import csv
import psycopg2
from psycopg2 import sql
import logging
from datetime import datetime

DB_CONFIG = {
    "dbname": "dbname",
    "user": "db_user",
    "password": "db_password",
    "host": "localhost",
    "port": "5432",
}

CSV_FILE_PATH = "sample_data.csv"

logging.basicConfig(
    filename="data_refresh_log.log",
    level=logging.INFO,
    format="%(asctime)s - %(levelname)s - %(message)s",
)

def connect_to_db():
    try:
        conn = psycopg2.connect(**DB_CONFIG)
        return conn
    except Exception as e:
        logging.error(f"Error connecting to the database: {e}")
        return None

def insert_data_from_csv(csv_file_path):
    conn = connect_to_db()
    if not conn:
        return

    cur = conn.cursor()

    try:
        with open(csv_file_path, mode="r") as csv_file:
            csv_reader = csv.DictReader(csv_file)
            for row in csv_reader:
                order_id = int(row["Order ID"])
                product_id = row["Product ID"]
                customer_id = row["Customer ID"]
                product_name = row["Product Name"]
                category = row["Category"]
                region = row["Region"]
                date_of_sale = row["Date of Sale"]
                quantity_sold = int(row["Quantity Sold"])
                unit_price = float(row["Unit Price"])
                discount = float(row["Discount"])
                shipping_cost = float(row["Shipping Cost"])
                payment_method = row["Payment Method"]
                customer_name = row["Customer Name"]
                customer_email = row["Customer Email"]
                customer_address = row["Customer Address"]

                cur.execute(
                    sql.SQL(
                        """
                        WITH new_customer AS (
                            INSERT INTO customers (name, email, address)
                            VALUES (%s, %s, %s)
                            ON CONFLICT (email) DO NOTHING
                            RETURNING id
                        )
                        SELECT id FROM new_customer
                        UNION
                        SELECT id FROM customers WHERE email = %s;
                        """
                    ),
                    (customer_name, customer_email, customer_address, customer_email),
                )
                customer_id_db = cur.fetchone()[0] 

                cur.execute(
                    sql.SQL(
                        """
                        WITH new_product AS (
                            INSERT INTO products (name, category, unit_price)
                            VALUES (%s, %s, %s)
                            ON CONFLICT (name, category) DO NOTHING
                            RETURNING id
                        )
                        SELECT id FROM new_product
                        UNION
                        SELECT id FROM products WHERE name = %s AND category = %s;
                        """
                    ),
                    (product_name, category, unit_price, product_name, category),
                )
                product_id_db = cur.fetchone()[0] 

                cur.execute(
                    sql.SQL(
                        """
                        INSERT INTO orders (id, customer_id, date_of_sale, payment_method, region, shipping_cost)
                        VALUES (%s, %s, %s, %s, %s, %s)
                        ON CONFLICT (id) DO NOTHING;
                        """
                    ),
                    (order_id, customer_id_db, date_of_sale, payment_method, region, shipping_cost),
                )

                cur.execute(
                    sql.SQL(
                        """
                        INSERT INTO order_items (order_id, product_id, quantity_sold, discount)
                        VALUES (%s, %s, %s, %s)
                        ON CONFLICT (order_id, product_id) DO NOTHING;
                        """
                    ),
                    (order_id, product_id_db, quantity_sold, discount),
                )

        conn.commit()
        logging.info("Data inserted successfully!")

    except Exception as e:
        logging.error(f"Error inserting data: {e}")
        conn.rollback()

    finally:
        cur.close()
        conn.close()

if __name__ == "__main__":
    logging.info("Starting data refresh process...")
    insert_data_from_csv(CSV_FILE_PATH)
    logging.info("Data refresh process completed.")