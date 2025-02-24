use log::info;
use serde_json::Value;
use serde_json::json;

use crate::repository::database_connection::db_connection;
use crate::models::request_models::*;
use tokio_postgres::{Client, NoTls, Row};

 
pub async fn sp_total_revenue(data: GlobalConfigModel, req_stamp: f64,start_date:&str,end_date:&str) -> Result<String, Box<dyn std::error::Error>> {
    let client: Client = db_connection(&data).await?;
    let qry = "SELECT * FROM total_revenue($1, $2);";
    if data.io_log == 0 {
        info!("STAMP: {:?}, DB-REQUEST, QUERY: {:?}, PARAMS:  {:?}, {:?}", 
              req_stamp, qry, start_date, end_date);
    }
    let row = client.query_one(qry, &[&start_date, &end_date]).await?;
    if data.io_log ==0{info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&row);}
    if row.is_empty() {
        return Err("No results returned".into());
    }
    let status_id: i32 = row.get(0);
    if status_id!=0{
        let out_json = json!({
            "message": "Invaild Input",
        });
        return Ok(serde_json::to_string(&out_json)?)
    }
    let total_revenue: f64 = row.get(1);
    let out_json = json!({
        "total_revenue": total_revenue,
    });
    Ok(serde_json::to_string(&out_json)?)
}
pub async fn sp_total_revenue_by_product(data: GlobalConfigModel, req_stamp: f64,start_date:&str,end_date:&str) -> Result<String, Box<dyn std::error::Error>> {
    let client: Client = db_connection(&data).await?;
    let qry = "SELECT * FROM total_revenue_by_product($1, $2);";
    if data.io_log == 0 {
        info!("STAMP: {:?}, DB-REQUEST, QUERY: {:?}, PARAMS:  {:?}, {:?}", 
              req_stamp, qry, start_date, end_date);
    }
    let rows = client.query(qry, &[&start_date, &end_date]).await?;
    if data.io_log == 0 {info!("STAMP: {:?}, DB-RESPONSE, RESULT-SET: {:?}", req_stamp, &rows);}
    if rows.is_empty() {return Err("No results returned".into());}
    let status_id: i32 = rows[0].get(0);
    if status_id!=0{
        let out_json = json!({
            "message": "Invaild Input",
        });
        return Ok(serde_json::to_string(&out_json)?)
    }
    let mut result_set = Vec::new();
    for row in rows {
        let product_name: String = row.get(1);
        let total_revenue: f64 = row.get(2);
        result_set.push(json!({
            "product_name": product_name,
            "total_revenue": total_revenue,
        }));
    }
    let out_json = json!({ "by_products": result_set });
    Ok(serde_json::to_string(&out_json)?)
}
pub async fn sp_total_revenue_by_category(data: GlobalConfigModel, req_stamp: f64,start_date:&str,end_date:&str) -> Result<String, Box<dyn std::error::Error>> {
    let client: Client = db_connection(&data).await?;
    let qry = "SELECT * FROM total_revenue_by_category($1, $2);";
    if data.io_log == 0 {
        info!("STAMP: {:?}, DB-REQUEST, QUERY: {:?}, PARAMS:  {:?}, {:?}", 
              req_stamp, qry, start_date, end_date);
    }
    let rows = client.query(qry, &[&start_date, &end_date]).await?;
    if data.io_log == 0 {info!("STAMP: {:?}, DB-RESPONSE, RESULT-SET: {:?}", req_stamp, &rows);}
    if rows.is_empty() {return Err("No results returned".into());}
    let status_id: i32 = rows[0].get(0);
    if status_id!=0{
        let out_json = json!({
            "message": "Invaild Input",
        });
        return Ok(serde_json::to_string(&out_json)?)
    }
    let mut result_set = Vec::new();
    for row in rows {
        let category: String = row.get(1);
        let total_revenue: f64 = row.get(2);
        result_set.push(json!({
            "category": category,
            "total_revenue": total_revenue,
        }));
    }
    let out_json = json!({ "by_category": result_set });
    Ok(serde_json::to_string(&out_json)?)
}
pub async fn sp_total_revenue_by_region(data: GlobalConfigModel, req_stamp: f64,start_date:&str,end_date:&str) -> Result<String, Box<dyn std::error::Error>> {
    let client: Client = db_connection(&data).await?;
    let qry = "SELECT * FROM total_revenue_by_region($1, $2);";
    if data.io_log == 0 {
        info!("STAMP: {:?}, DB-REQUEST, QUERY: {:?}, PARAMS:  {:?}, {:?}", 
              req_stamp, qry, start_date, end_date);
    }
    let rows = client.query(qry, &[&start_date, &end_date]).await?;
    if data.io_log == 0 {info!("STAMP: {:?}, DB-RESPONSE, RESULT-SET: {:?}", req_stamp, &rows);}
    if rows.is_empty() {return Err("No results returned".into());}
    let status_id: i32 = rows[0].get(0);
    if status_id!=0{
        let out_json = json!({
            "message": "Invaild Input",
        });
        return Ok(serde_json::to_string(&out_json)?)
    }
    let mut result_set = Vec::new();
    for row in rows {
        let region: String = row.get(1);
        let total_revenue: f64 = row.get(2);
        result_set.push(json!({
            "region": region,
            "total_revenue": total_revenue,
        }));
    }
    let out_json = json!({ "by_region": result_set });
    Ok(serde_json::to_string(&out_json)?)
}
pub async fn sp_total_customers(data: GlobalConfigModel, req_stamp: f64,start_date:&str,end_date:&str) -> Result<String, Box<dyn std::error::Error>> {
    let client: Client = db_connection(&data).await?;
    let qry = "SELECT * FROM total_customers($1, $2);";
    if data.io_log == 0 {
        info!("STAMP: {:?}, DB-REQUEST, QUERY: {:?}, PARAMS:  {:?}, {:?}", 
              req_stamp, qry, start_date, end_date);
    }
    let row = client.query_one(qry, &[&start_date, &end_date]).await?;
    if data.io_log ==0{info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&row);}
    if row.is_empty() {
        return Err("No results returned".into());
    }
    let status_id: i32 = row.get(0);
    if status_id!=0{
        let out_json = json!({
            "message": "Invaild Input",
        });
        return Ok(serde_json::to_string(&out_json)?)
    }
    let total_customers: i32 = row.get(1);
    let out_json = json!({
        "total_customers": total_customers,
    });
    Ok(serde_json::to_string(&out_json)?)
}
pub async fn sp_total_orders(data: GlobalConfigModel, req_stamp: f64,start_date:&str,end_date:&str) -> Result<String, Box<dyn std::error::Error>> {
    let client: Client = db_connection(&data).await?;
    let qry = "SELECT * FROM total_orders($1, $2);";
    if data.io_log == 0 {
        info!("STAMP: {:?}, DB-REQUEST, QUERY: {:?}, PARAMS:  {:?}, {:?}", 
              req_stamp, qry, start_date, end_date);
    }
    let row = client.query_one(qry, &[&start_date, &end_date]).await?;
    if data.io_log ==0{info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&row);}
    if row.is_empty() {
        return Err("No results returned".into());
    }
    let status_id: i32 = row.get(0);
    if status_id!=0{
        let out_json = json!({
            "message": "Invaild Input",
        });
        return Ok(serde_json::to_string(&out_json)?)
    }
    let total_orders: i32 = row.get(1);
    let out_json = json!({
        "total_orders": total_orders,
    });
    Ok(serde_json::to_string(&out_json)?)
}
pub async fn sp_average_order_value(data: GlobalConfigModel, req_stamp: f64,start_date:&str,end_date:&str) -> Result<String, Box<dyn std::error::Error>> {
    let client: Client = db_connection(&data).await?;
    let qry = "SELECT * FROM average_order_value($1, $2);";
    if data.io_log == 0 {
        info!("STAMP: {:?}, DB-REQUEST, QUERY: {:?}, PARAMS:  {:?}, {:?}", 
              req_stamp, qry, start_date, end_date);
    }
    let row = client.query_one(qry, &[&start_date, &end_date]).await?;
    if data.io_log ==0{info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&row);}
    if row.is_empty() {
        return Err("No results returned".into());
    }
    let status_id: i32 = row.get(0);
    if status_id!=0{
        let out_json = json!({
            "message": "Invaild Input",
        });
        return Ok(serde_json::to_string(&out_json)?)
    }
    let average_order_value: f64 = row.get(1);
    let out_json = json!({
        "average_order_value": average_order_value,
    });
    Ok(serde_json::to_string(&out_json)?)
}
pub async fn sp_customer_lifetime_value(data: GlobalConfigModel, req_stamp: f64,start_date:&str,end_date:&str) -> Result<String, Box<dyn std::error::Error>> {
    let client: Client = db_connection(&data).await?;
    let qry = "SELECT * FROM customer_lifetime_value($1, $2);";
    if data.io_log == 0 {
        info!("STAMP: {:?}, DB-REQUEST, QUERY: {:?}, PARAMS:  {:?}, {:?}", 
              req_stamp, qry, start_date, end_date);
    }
    let rows = client.query(qry, &[&start_date, &end_date]).await?;
    if data.io_log == 0 {info!("STAMP: {:?}, DB-RESPONSE, RESULT-SET: {:?}", req_stamp, &rows);}
    if rows.is_empty() {return Err("No results returned".into());}
    let status_id: i32 = rows[0].get(0);
    if status_id!=0{
        let out_json = json!({
            "message": "Invaild Input",
        });
        return Ok(serde_json::to_string(&out_json)?)
    }
    let mut result_set = Vec::new();
    for row in rows {
        let customer_name: String = row.get(1);
        let lifetime_value: f64 = row.get(2);
        result_set.push(json!({
            "customer_name": customer_name,
            "lifetime_value": lifetime_value,
        }));
    }
    let out_json = json!({ "customer_lifetime_value": result_set });
    Ok(serde_json::to_string(&out_json)?)
        
}
pub async fn sp_top_n_products_overall(data: GlobalConfigModel,req_stamp: f64,number: i32,start_date: &str,end_date: &str,) -> Result<String, Box<dyn std::error::Error>> {
    let client: Client = db_connection(&data).await?;
    let qry = "SELECT * FROM top_n_products_overall($1, $2, $3);";
    if data.io_log == 0 {
        info!("STAMP: {:?}, DB-REQUEST, QUERY: {:?}, PARAMS: {:?}, {:?}, {:?}", 
              req_stamp, qry, number, start_date, end_date);
    }
    let rows = client.query(qry, &[&number, &start_date, &end_date]).await?;
    if data.io_log == 0 {
        info!("STAMP: {:?}, DB-RESPONSE, RESULT-SET: {:?}", req_stamp, rows);
    }
    if rows.is_empty() {
        return Err("No results returned".into());
    }
    let status_id: i32 = rows[0].get(0);
    if status_id != 0 {
        let out_json = json!({
            "message": "Invalid Input"
        });
        return Ok(serde_json::to_string(&out_json)?);
    }
    let mut result_set = Vec::new();
    for row in rows {
        let product_name: String = row.get(1);
        let total_quantity_sold: i64 = row.get(2);
        result_set.push(json!({
            "product_name": product_name,
            "total_quantity_sold": total_quantity_sold,
        }));
    }
    let out_json = json!({ "overall": result_set });
    Ok(serde_json::to_string(&out_json)?)
}

pub async fn sp_top_n_products_by_category(data: GlobalConfigModel, req_stamp: f64,numbers:i32,filterby:&str,start_date:&str,end_date:&str) -> Result<String, Box<dyn std::error::Error>> {
    let client: Client = db_connection(&data).await?;
    let qry = "SELECT * FROM top_n_products_by_category($1, $2, $3, $4);";
    if data.io_log == 0 {
        info!("STAMP: {:?}, DB-REQUEST, QUERY: {:?}, PARAMS: {:?}, {:?}, {:?}", 
              req_stamp, qry, numbers, start_date, end_date);
    }
    let rows = client.query(qry, &[&numbers, &filterby, &start_date, &end_date]).await?;
    if data.io_log == 0 {
        info!("STAMP: {:?}, DB-RESPONSE, RESULT-SET: {:?}", req_stamp, rows);
    }
    if rows.is_empty() {
        return Err("No results returned".into());
    }
    let status_id: i32 = rows[0].get(0);
    if status_id != 0 {
        let out_json = json!({
            "message": "Invalid Input"
        });
        return Ok(serde_json::to_string(&out_json)?);
    }
    let mut result_set = Vec::new();
    for row in rows {
        let product_name: String = row.get(1);
        let total_quantity_sold: i64 = row.get(2);
        result_set.push(json!({
            "product_name": product_name,
            "total_quantity_sold": total_quantity_sold,
        }));
    }
    let out_json = json!({ "by_category": result_set });
    Ok(serde_json::to_string(&out_json)?)
}
pub async fn sp_top_n_products_by_region(data: GlobalConfigModel, req_stamp: f64,numbers:i32,filterby:&str,start_date:&str,end_date:&str) -> Result<String, Box<dyn std::error::Error>> {
    let client: Client = db_connection(&data).await?;
    let qry = "SELECT * FROM top_n_products_by_region($1, $2, $3, $4);";
    if data.io_log == 0 {
        info!("STAMP: {:?}, DB-REQUEST, QUERY: {:?}, PARAMS: {:?}, {:?}, {:?}", 
              req_stamp, qry, numbers, start_date, end_date);
    }
    let rows = client.query(qry, &[&numbers, &filterby, &start_date, &end_date]).await?;
    if data.io_log == 0 {
        info!("STAMP: {:?}, DB-RESPONSE, RESULT-SET: {:?}", req_stamp, rows);
    }
    if rows.is_empty() {
        return Err("No results returned".into());
    }
    let status_id: i32 = rows[0].get(0);
    if status_id != 0 {
        let out_json = json!({
            "message": "Invalid Input"
        });
        return Ok(serde_json::to_string(&out_json)?);
    }
    let mut result_set = Vec::new();
    for row in rows {
        let product_name: String = row.get(1);
        let total_quantity_sold: i64 = row.get(2);
        result_set.push(json!({
            "product_name": product_name,
            "total_quantity_sold": total_quantity_sold,
        }));
    }
    let out_json = json!({ "by_region": result_set });
    Ok(serde_json::to_string(&out_json)?)
}