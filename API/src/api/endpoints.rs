use actix_web::{post,get, web,Result, HttpResponse,Responder,HttpRequest};
use chrono::Utc;
use log::error;
use log::info;
use serde_json::{Value,json};
use crate::models::request_models::*;
use crate::repository::database_functions::*;
use tokio_postgres::{NoTls};



#[get("/get_version/")]
async fn get_version_handler(req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let parsed = json!({
        "date":"2025-02-21",
        "api_version":"1.0.0"
    });
    return Ok(web::Json(parsed)) 
    
}
#[post("/total_revenue/")]
async fn total_revenue(data: web::Data<( GlobalConfigModel,)>,info:web::Json<DateModel>)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "total_revenue";
    if data.0.io_log ==0{
        let req_data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,req_data);
    }
    let result = sp_total_revenue(data.0.clone(),req_stamp,&info.start_date,&info.end_date).await;
    match result {
        Ok(x)=>{
            let parsed: Value = serde_json::from_str(&x)?;
            if data.0.io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(HttpResponse::Ok().json(parsed));
        }
        Err(e) =>{
            if data.0.error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"Status_id\":\"500\",\"Message\":\"Internal Server Error\"}")?;
            return Ok(HttpResponse::InternalServerError().json(parsed));
        }
    }   
}
#[post("/total_revenue_by_product/")]
async fn total_revenue_by_product(data: web::Data<( GlobalConfigModel,)>,info:web::Json<DateModel>)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "total_revenue_by_product";
    if data.0.io_log ==0{
        let req_data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,req_data);
    }
    let result = sp_total_revenue_by_product(data.0.clone(),req_stamp,&info.start_date,&info.end_date).await;
    match result {
        Ok(x)=>{
            let parsed: Value = serde_json::from_str(&x)?;
            if data.0.io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(HttpResponse::Ok().json(parsed));
        }
        Err(e) =>{
            if data.0.error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"Status_id\":\"500\",\"Message\":\"Internal Server Error\"}")?;
            return Ok(HttpResponse::InternalServerError().json(parsed));
        }
    }
    
}
#[post("/total_revenue_by_category/")]
async fn total_revenue_by_category(data: web::Data<( GlobalConfigModel,)>,info:web::Json<DateModel>)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "total_revenue_by_category";
    if data.0.io_log ==0{
        let req_data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,req_data);
    }
    let result = sp_total_revenue_by_category(data.0.clone(),req_stamp,&info.start_date,&info.end_date).await;
    match result {
        Ok(x)=>{
            let parsed: Value = serde_json::from_str(&x)?;
            if data.0.io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(HttpResponse::Ok().json(parsed));
        }
        Err(e) =>{
            if data.0.error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"Status_id\":\"500\",\"Message\":\"Internal Server Error\"}")?;
            return Ok(HttpResponse::InternalServerError().json(parsed));
        }
    }
    
}
#[post("/total_revenue_by_region/")]
async fn total_revenue_by_region(data: web::Data<( GlobalConfigModel,)>,info:web::Json<DateModel>)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "total_revenue_by_region";
    if data.0.io_log ==0{
        let req_data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,req_data);
    }
    let result = sp_total_revenue_by_region(data.0.clone(),req_stamp,&info.start_date,&info.end_date).await;
    match result {
        Ok(x)=>{
            let parsed: Value = serde_json::from_str(&x)?;
            if data.0.io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(HttpResponse::Ok().json(parsed));
        }
        Err(e) =>{
            if data.0.error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"Status_id\":\"500\",\"Message\":\"Internal Server Error\"}")?;
            return Ok(HttpResponse::InternalServerError().json(parsed));
        }
    }
    
}
#[post("/total_customers/")]
async fn total_customers(data: web::Data<( GlobalConfigModel,)>,info:web::Json<DateModel>)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "total_customers";
    if data.0.io_log ==0{
        let req_data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,req_data);
    }
    let result = sp_total_customers(data.0.clone(),req_stamp,&info.start_date,&info.end_date).await;
    match result {
        Ok(x)=>{
            let parsed: Value = serde_json::from_str(&x)?;
            if data.0.io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(HttpResponse::Ok().json(parsed));
        }
        Err(e) =>{
            if data.0.error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"Status_id\":\"500\",\"Message\":\"Internal Server Error\"}")?;
            return Ok(HttpResponse::InternalServerError().json(parsed));
        }
    }
    
}
#[post("/total_orders/")]
async fn total_orders(data: web::Data<( GlobalConfigModel,)>,info:web::Json<DateModel>)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "total_orders";
    if data.0.io_log ==0{
        let req_data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,req_data);
    }
    let result = sp_total_orders(data.0.clone(),req_stamp,&info.start_date,&info.end_date).await;
    match result {
        Ok(x)=>{
            let parsed: Value = serde_json::from_str(&x)?;
            if data.0.io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(HttpResponse::Ok().json(parsed));
        }
        Err(e) =>{
            if data.0.error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"Status_id\":\"500\",\"Message\":\"Internal Server Error\"}")?;
            return Ok(HttpResponse::InternalServerError().json(parsed));
        }
    }
    
}
#[post("/average_order_value/")]
async fn average_order_value(data: web::Data<( GlobalConfigModel,)>,info:web::Json<DateModel>)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "average_order_value";
    if data.0.io_log ==0{
        let req_data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,req_data);
    }
    let result = sp_average_order_value(data.0.clone(),req_stamp,&info.start_date,&info.end_date).await;
    match result {
        Ok(x)=>{
            let parsed: Value = serde_json::from_str(&x)?;
            if data.0.io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(HttpResponse::Ok().json(parsed));
        }
        Err(e) =>{
            if data.0.error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"Status_id\":\"500\",\"Message\":\"Internal Server Error\"}")?;
            return Ok(HttpResponse::InternalServerError().json(parsed));
        }
    }
    
}
#[post("/customer_lifetime_value/")]
async fn customer_lifetime_value(data: web::Data<( GlobalConfigModel,)>,info:web::Json<DateModel>)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "customer_lifetime_value";
    if data.0.io_log ==0{
        let req_data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,req_data);
    }
    let result = sp_customer_lifetime_value(data.0.clone(),req_stamp,&info.start_date,&info.end_date).await;
    match result {
        Ok(x)=>{
            let parsed: Value = serde_json::from_str(&x)?;
            if data.0.io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(HttpResponse::Ok().json(parsed));
        }
        Err(e) =>{
            if data.0.error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"Status_id\":\"500\",\"Message\":\"Internal Server Error\"}")?;
            return Ok(HttpResponse::InternalServerError().json(parsed));
        }
    }
    
}
#[post("/top_n_products_overall/")]
async fn top_n_products_overall(data: web::Data<( GlobalConfigModel,)>,info:web::Json<TopNOverall>)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "top_n_products_overall";
    if data.0.io_log ==0{
        let req_data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,req_data);
    }
    let result = sp_top_n_products_overall(data.0.clone(),req_stamp,info.numbers,&info.start_date,&info.end_date).await;
    match result {
        Ok(x)=>{
            let parsed: Value = serde_json::from_str(&x)?;
            if data.0.io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(HttpResponse::Ok().json(parsed));
        }
        Err(e) =>{
            if data.0.error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"Status_id\":\"500\",\"Message\":\"Internal Server Error\"}")?;
            return Ok(HttpResponse::InternalServerError().json(parsed));
        }
    }
    
}
#[post("/top_n_products_by_category/")]
async fn top_n_products_by_category(data: web::Data<( GlobalConfigModel,)>,info:web::Json<TopNFilter>)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "top_n_products_by_category";
    if data.0.io_log ==0{
        let req_data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,req_data);
    }
    let result = sp_top_n_products_by_category(data.0.clone(),req_stamp,info.numbers,&info.filterby,&info.start_date,&info.end_date).await;
    match result {
        Ok(x)=>{
            let parsed: Value = serde_json::from_str(&x)?;
            if data.0.io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(HttpResponse::Ok().json(parsed));
        }
        Err(e) =>{
            if data.0.error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"Status_id\":\"500\",\"Message\":\"Internal Server Error\"}")?;
            return Ok(HttpResponse::InternalServerError().json(parsed));
        }
    }
    
}
#[post("/top_n_products_by_region/")]
async fn top_n_products_by_region(data: web::Data<( GlobalConfigModel,)>,info:web::Json<TopNFilter>)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "top_n_products_by_region";
    if data.0.io_log ==0{
        let req_data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,req_data);
    }
    let result = sp_top_n_products_by_region(data.0.clone(),req_stamp,info.numbers,&info.filterby,&info.start_date,&info.end_date).await;
    match result {
        Ok(x)=>{
            let parsed: Value = serde_json::from_str(&x)?;
            if data.0.io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(HttpResponse::Ok().json(parsed));
        }
        Err(e) =>{
            if data.0.error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"Status_id\":\"500\",\"Message\":\"Internal Server Error\"}")?;
            return Ok(HttpResponse::InternalServerError().json(parsed));
        }
    }
    
}