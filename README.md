# Sales Monitoring API

## Overview
Sales Monitoring API provides various endpoints to analyze sales data, including total revenue, customer statistics, order trends, and product performance over a given date range.

## Database Schema
*(Include the JPEG image of the database schema in your repository and reference it here.)*

## API Endpoints

### 1. Total Revenue
- **Endpoint:** `POST /v1/total_revenue/`
- **Request Body:**
  ```json
  {
     "start_date": "2023-12-01",
     "end_date": "2024-12-31"
  }
  ```

### 2. Total Customers
- **Endpoint:** `POST /v1/total_customers/`
- **Request Body:**
  ```json
  {
     "start_date": "2023-12-01",
     "end_date": "2024-12-31"
  }
  ```

### 3. Total Orders
- **Endpoint:** `POST /v1/total_orders/`
- **Request Body:**
  ```json
  {
     "start_date": "2023-12-01",
     "end_date": "2024-12-31"
  }
  ```

### 4. Average Order Value
- **Endpoint:** `POST /v1/average_order_value/`
- **Request Body:**
  ```json
  {
     "start_date": "2023-12-01",
     "end_date": "2024-12-31"
  }
  ```

### 5. Total Revenue by Product
- **Endpoint:** `POST /v1/total_revenue_by_product/`
- **Request Body:**
  ```json
  {
     "start_date": "2023-12-01",
     "end_date": "2024-12-31"
  }
  ```

### 6. Total Revenue by Category
- **Endpoint:** `POST /v1/total_revenue_by_category/`
- **Request Body:**
  ```json
  {
     "start_date": "2023-12-01",
     "end_date": "2024-12-31"
  }
  ```

### 7. Total Revenue by Region
- **Endpoint:** `POST /v1/total_revenue_by_region/`
- **Request Body:**
  ```json
  {
     "start_date": "2023-12-01",
     "end_date": "2024-12-31"
  }
  ```

### 8. Customer Lifetime Value
- **Endpoint:** `POST /v1/customer_lifetime_value/`
- **Request Body:**
  ```json
  {
     "start_date": "2023-12-01",
     "end_date": "2024-12-31"
  }
  ```

### 9. Top N Products Overall
- **Endpoint:** `POST /v1/top_n_products_overall/`
- **Request Body:**
  ```json
  {
      "numbers": 3,
      "start_date": "2023-12-01",
      "end_date": "2024-12-31"
  }
  ```

### 10. Top N Products by Category
- **Endpoint:** `POST /v1/top_n_products_by_category/`
- **Request Body:**
  ```json
  {
      "numbers": 2,
      "filterby": "Electronics",
      "start_date": "2023-12-01",
      "end_date": "2024-12-31"
  }
  ```

### 11. Top N Products by Region
- **Endpoint:** `POST /v1/top_n_products_by_region/`
- **Request Body:**
  ```json
  {
      "numbers": 2,
      "filterby": "North America",
      "start_date": "2023-12-01",
      "end_date": "2024-12-31"
  }
  ```

## Setup Instructions

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd sales-monitoring-api
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Run the API:
   ```bash
   cargo run
   ```

4. API is available at `http://localhost:8100/`

## Contributing
Feel free to submit issues and pull requests to improve this project.

## License
This project is licensed under the MIT License.

