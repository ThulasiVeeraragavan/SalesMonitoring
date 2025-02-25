# Sales Monitoring Project

## Database Schema
![Database Schema](database_schema.jpeg)

## Setup Instructions

### Step 1: Database Setup
Ensure all functions provided in the `db_script` directory are executed in the database.

### Step 2: CSV Loader
**Software Requirement:** Python 3.8 or above

#### Installation
Run the following command to install the required dependencies:
```sh
pip install -r req.txt
```

#### Running the CSV Loader
- **Manual Run:**
  ```sh
  python3 csv_up.py
  ```
- **Automatic Execution Every 5 Minutes:**
  ```sh
  python3 data_refresh_scheduler.py
  ```
- Refresh logs are recorded in `data_refresh_log.log`.

  ### Step 3: API Server
The API server is located in the `API` directory and is built using Rust.

#### Rust Installation
Install Rust using the official guide: [Rust Installation](https://www.rust-lang.org/tools/install)

#### Configuration
- `database_config.json` contains the configuration settings.
- Toggle log settings:
  - **Enable Environment Log:** Set `toggle_log` to `1`.
  - **Enable File Log:** Set `toggle_log` to `0`.
- Logs are stored in the file specified in `log_file_path`.

#### Running the Rust API
- **API Run:**
  ```sh
  cargo run
  ```

### Step 4: API Endpoints
**Postman collection is available in the `postman_collection` directory.**

#### API Base URL
```
http://localhost:8100/v1/
```

#### Available API Endpoints

| Method | Endpoint | Request Body |
|--------|---------|--------------|
| **POST** | `/total_revenue/` | `{ "start_date":"2023-12-01", "end_date":"2024-12-31" }` |
| **POST** | `/total_customers/` | `{ "start_date":"2023-12-01", "end_date":"2024-12-31" }` |
| **POST** | `/total_orders/` | `{ "start_date":"2023-12-01", "end_date":"2024-12-31" }` |
| **POST** | `/average_order_value/` | `{ "start_date":"2023-12-01", "end_date":"2024-12-31" }` |
| **POST** | `/total_revenue_by_product/` | `{ "start_date":"2023-12-01", "end_date":"2024-12-31" }` |
| **POST** | `/total_revenue_by_category/` | `{ "start_date":"2023-12-01", "end_date":"2024-12-31" }` |
| **POST** | `/total_revenue_by_region/` | `{ "start_date":"2023-12-01", "end_date":"2024-12-31" }` |
| **POST** | `/customer_lifetime_value/` | `{ "start_date":"2023-12-01", "end_date":"2024-12-31" }` |
| **POST** | `/top_n_products_overall/` | `{ "numbers":3, "start_date":"2023-12-01", "end_date":"2024-12-31" }` |
| **POST** | `/top_n_products_by_category/` | `{ "numbers":2, "filterby":"Electronics", "start_date":"2023-12-01", "end_date":"2024-12-31" }` |
| **POST** | `/top_n_products_by_region/` | `{ "numbers":2, "filterby":"North America", "start_date":"2023-12-01", "end_date":"2024-12-31" }` |



---

**Ensure all dependencies are installed and configurations are properly set before running the system.**

