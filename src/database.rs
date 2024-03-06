use dotenv::dotenv;
use std::env;
use tokio_postgres::NoTls;
use tokio;

pub async fn connect_to_db() -> Result<tokio_postgres::Client, Box<dyn std::error::Error>> {
    // Load .env file
    dotenv().ok();

    // Get database URL from .env file
    let database_url = env::var("DATABASE_URL")?;
    
    // Connect to the database
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

pub async fn insert_into_litemapindex(
    db_client: &tokio_postgres::Client,
    inscriptionid: &str,
    inscriptiontext: &str,
    indexnum: i64,
    extensionendpoint: &str,
    inscrptionnum: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let stmt = "
        INSERT INTO nft.litemapindex (inscriptionid, inscriptiontext, indexnum, extensionendpoint,inscrptionnum)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (inscriptiontext) DO NOTHING;
    ";
    db_client.execute(stmt, &[&inscriptionid, &inscriptiontext, &indexnum, &extensionendpoint,&inscrptionnum]).await?;
    Ok(())
}


pub async fn insert_into_textindex(
    db_client: &tokio_postgres::Client,
    inscriptionid: &str,
    inscriptiontext: &str,
    inscrptionnum: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let stmt = "
        INSERT INTO nft.litemapindex (inscriptionid, inscriptiontext,inscrptionnum)
        VALUES ($1, $2, $3)
        ON CONFLICT (inscriptiontext) DO NOTHING;
    ";
    db_client.execute(stmt, &[&inscriptionid, &inscriptiontext,&inscrptionnum]).await?;
    Ok(())
}
