use crate::models::request_models::GlobalConfigModel;



use tokio_postgres::{NoTls, Client, Error};

pub async fn db_connection(data: &GlobalConfigModel) -> Result<Client, Error> {
    let connection_str = format!(
        "host={} port={} dbname={} user={} password={}",
        data.db_host, data.db_port, data.db_name, data.db_user_name, data.db_password
    );

    let (client, connection) = tokio_postgres::connect(&connection_str, NoTls).await?;

    // Spawn the connection on a separate task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("PostgreSQL connection error: {}", e);
        }
    });

    Ok(client)
}