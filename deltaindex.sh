#!/bin/bash

# Database connection details
DATABASE_URL=postgres://username:password@host:port/database


# Parse the DATABASE_URL into components
# Note: This simplistic parsing assumes your password, host, and other components do not contain special characters like @, /, or : that would break the parsing.
PGUSER=$(echo $DATABASE_URL | grep -oP '^postgres://\K[^:]+')
PGPASSWORD=$(echo $DATABASE_URL | grep -oP '://[^:]+:\K[^@]+')
PGHOST=$(echo $DATABASE_URL | grep -oP '@\K[^:]+')
PGPORT=$(echo $DATABASE_URL | grep -oP ':\K[0-9]+(?=/)')
PGDATABASE=$(echo $DATABASE_URL | grep -oP '/\K[^/]+$')


export PGUSER PGPASSWORD PGHOST PGPORT PGDATABASE

echo ${PGUSER}
echo ${PGPASSWORD}
echo ${PGHOST}
echo ${PGPORT}
echo ${PGDATABASE}

# Define file paths
TARGET_FILE="/home/weatherbot/Documents/lordinalindex/index.tsv"
SOURCE_FILE="/home/weatherbot/Documents/index.tsv"
TEMP_FILE="/home/weatherbot/Documents/lordinalindex/temp_index.tsv"

rm -f "${TEMP_FILE}" "${TARGET_FILE}"

# Extract the last inscription number from the database
LAST_INSCRIPTION=$(psql -t -c "select max(inscrptionnum) from nft.litemapindex;")



echo "${LAST_INSCRIPTION}"

# Check if LAST_INSCRIPTION is a valid number
#if ! echo "${LAST_INSCRIPTION}" | grep -qE '^[0-9]+$'; then
#    echo "Error: Last inscription number is not a valid number."
#    exit 1
#fi

# Extract records from the target file starting from the row with the last inscription number
#awk -v start="${LAST_INSCRIPTION}" 'BEGIN{FS=OFS="\t"} $1 >= start' "${SOURCE_FILE}" > "${TEMP_FILE}"
awk -v start="${LAST_INSCRIPTION}" 'BEGIN{FS=OFS="\t"} NR>1 && $1 >= start' "${SOURCE_FILE}" > "${TEMP_FILE}"

# Check if TEMP_FILE is not empty, then overwrite the TARGET_FILE with TEMP_FILE
if [ -s "${TEMP_FILE}" ]; then
    mv -f "${TEMP_FILE}" "${TARGET_FILE}" # Corrected to move to TARGET_FILE
    echo "The target file has been updated with the new delta."
else
    echo "No new records to add. The target file remains unchanged."
fi

# Cleanup
rm -f "${TEMP_FILE}"

