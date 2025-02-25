import logging
from csv_up import CSV_FILE_PATH, insert_data_from_csv
import schedule
import time

logging.basicConfig(
    filename="data_refresh_log.log",
    level=logging.INFO,
    format="%(asctime)s - %(levelname)s - %(message)s",
)

def refresh_data():
    """Function to refresh data by inserting data from CSV."""
    logging.info("Starting scheduled data refresh...")
    try:
        insert_data_from_csv(CSV_FILE_PATH)
        logging.info("Scheduled data refresh completed successfully.")
    except Exception as e:
        logging.error(f"Error during data refresh: {e}")

schedule.every(5).minutes.do(refresh_data)

logging.info("Data refresh scheduler started. Refreshing every 5 minutes...")
while True:
    schedule.run_pending()
    time.sleep(1)