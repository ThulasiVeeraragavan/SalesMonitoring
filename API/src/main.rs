mod api;
mod models;
mod repository;
mod encryptions;
use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use crate::api::routes::*;
use crate::models::request_models::*;
use crate::api::endpoints::get_version_handler;
use fern;
use chrono::Utc;
use actix_web::middleware::Logger;
use env_logger::Env;
use std::path::Path;
use std::fs::{self, File};
#[allow(non_snake_case)]


fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {

    let json_file_path= Path::new("./json_files/database_config.json");
    let file = File::open(json_file_path)?;
    let games:GlobalConfigModel=serde_json::from_reader(file)?;
    let path = Path::new(&games.log_file_path);
    if !path.exists() {
        match fs::create_dir_all(&path) {
            Ok(_) => println!("Directory created successfully."),
            Err(e) => eprintln!("Failed to create directory: {}", e),
        }
    } else {
        println!("Directory already exists.");
    }
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{:?} {} {}] {}",
                Utc::now(),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("tiberius", log::LevelFilter::Off)
        .level_for("actix_web", log::LevelFilter::Off)
        .level_for("amiquip", log::LevelFilter::Off)
        .chain(fern::DateBased::new(path, "%Y-%m-%d-hour-%H-api.log"))
        .apply()?;

    Ok(())
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("-------------------Starting Actix-Web Server-----------------");
    println!("-------------------Reading config file --------------------");
    let json_file_path= Path::new("./json_files/database_config.json");
    let file = File::open(json_file_path)?;
    let games:GlobalConfigModel=serde_json::from_reader(file)?;
    let toggle_log = games.toggle_log;
    let api_port = games.api_port;
    let conf_state = (games,);
    let app_data = web::Data::new(conf_state);


    if toggle_log==0{
        setup_logging().expect("failed to initialize logging.");
    }
    else {
        env_logger::init_from_env(Env::default().default_filter_or("debug"));
    }
    HttpServer::new(move || {
        let cors = Cors::permissive()
            .allowed_methods(vec!["GET", "POST","OPTIONS"])
            .supports_credentials();
        App::new()
            .app_data(app_data.clone())
        .wrap(cors)
        .wrap(Logger::default())
        .service(web::scope("/v1").configure(init_routes_v1))
        .service(get_version_handler)
    })
    .bind(("0.0.0.0", api_port))?
    .run()
    .await
}
