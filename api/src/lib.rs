use dotenvy::dotenv;
use std::env;
use sea_orm::{Database, DatabaseConnection};

pub mod handler;
pub mod models;

pub async fn establish_connection() -> DatabaseConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(&database_url)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
