use std::error::Error;
use std::fs::{self, File, write, read_to_string};
use csv::ReaderBuilder;
use reqwest;
use tokio;
mod database;
use database::connect_to_db;
// Use these if necessary; otherwise, you can comment them out to avoid warnings.
use database::insert_into_litemapindex;
use database::insert_into_textindex;
use tokio_postgres::Client; // Ensure you have the correct import for Client.

async fn process_file_content(
    db_client: &Client, // Make sure this matches your actual client type
    file_content: &str,
    middle_record: &str,
    first_record: &str,
) -> Result<(), Box<dyn Error>> {
    let trimmed_content = file_content.trim();
    let inscriptionnumbertext = first_record.parse::<i64>()
        .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Error parsing inscription number: {}", e))))?;

    insert_into_textindex(db_client, middle_record, trimmed_content, inscriptionnumbertext).await?;

    let words: Vec<&str> = trimmed_content.split_whitespace().collect();
    if words.len() == 1 {
        let periods_count = trimmed_content.matches('.').count();
        if periods_count == 1 {
            let parts: Vec<&str> = trimmed_content.split('.').collect();
            if parts.len() == 2 {
                let index = parts[0];
                let text = parts[1];
                println!("Processed text from file {}: Index: {}, Text: {}", middle_record, index, text);
                let index_num = index.parse::<i64>()
                    .map_err(|_| Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Error parsing index as i64")))?;

                insert_into_litemapindex(db_client, middle_record, trimmed_content, index_num, text, inscriptionnumbertext).await?;
            }
        }
    }
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_client = connect_to_db().await?;

    let row = db_client
        .query_one("SELECT version();", &[])
        .await?;

    let version: &str = row.get(0);

    println!("Connected to PostgreSQL server: {}", version);
    let file_path = "index.tsv";
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().delimiter(b'\t').from_reader(file);

    let client = reqwest::Client::new();
    let mut counter = 0;

    for result in rdr.records() {
        let record = match result {
            Ok(record) => record,
            Err(e) => {
                eprintln!("Error reading record: {}", e);
                continue;
            }
        };
        if record.len() >= 2 {
            let middle_record = record.get(1).unwrap_or_default();
            let first_record = record.get(0).unwrap_or_default();
            let content = client
                .get(format!("http://207.148.23.107/content/{}", middle_record))
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")
                .send()
                .await?
                .text()
                .await?;
            let file_path = format!("filestg/{}.txt", middle_record);
            fs::create_dir_all("filestg")?;
            write(&file_path, &content)?;
            let file_content = read_to_string(&file_path)?;
            let line_count = file_content.lines().count();
            if line_count == 1 {
                if let Err(e) = process_file_content(&db_client, &file_content, middle_record, first_record).await {
                    eprintln!("Error processing file {}: {}", middle_record, e);
                }
            }
            if let Err(e) = fs::remove_file(&file_path) {
                eprintln!("Error deleting file {}: {}", middle_record, e);
            }
        }
        counter += 1;
        if counter % 10000 == 0 {
            println!("Processed {} records", counter);
        }
    }
    Ok(())
}
