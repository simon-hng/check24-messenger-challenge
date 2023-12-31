use actix::Actor;
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware, web, App, HttpServer};
use dotenvy::dotenv;
use entity::app::AppState;
use migration::{Migrator, MigratorTrait};
use resource::*;
use sea_orm::Database;
use std::env;

mod resource;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let secret_key = Key::from(secret_key.as_bytes());

    let client_url = env::var("CLIENT_URL").expect("Client URL must be set");

    // establish connection to database and apply migrations
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Database::connect(&database_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let redis_connection_string = env::var("REDIS_URL").expect("REDIS_URL must be set");

    // set up applications state
    // keep a count of the number of visitors
    let app_state = AppState { conn };
    let message_server = service::chat::server::NotificationServer::new(secret_key.clone()).start();

    let store = RedisSessionStore::new(redis_connection_string)
        .await
        .expect("Failed to connect to redis");

    log::info!("starting HTTP server on port 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::JsonConfig::default().limit(20_971_520)) // 20MB
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(message_server.clone()))
            .wrap(middleware::Logger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(store.clone(), secret_key.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin(&client_url.to_string())
                    // TODO Configure cors correctly
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin()
                    .supports_credentials(),
            )
            .configure(account::init_service)
            .configure(auth::init_service)
            .configure(conversation::init_service)
            .configure(notification::init_service)
            .configure(review::init_service)
            .configure(enquiry::init_service)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
