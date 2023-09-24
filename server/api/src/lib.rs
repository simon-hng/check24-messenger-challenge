use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware, web, App, HttpServer};
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use resource::{auth, conversation, message};
use sea_orm::{Database, DatabaseConnection};
use std::env;

mod resource;

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let secret_key = Key::from(secret_key.as_bytes());

    // establish connection to database and apply migrations
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Database::connect(&database_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let redis_connection_string = env::var("REDIS_URL").expect("REDIS_URL must be set");

    // set up applications state
    // keep a count of the number of visitors
    let app_state = AppState { conn };

    let store = RedisSessionStore::new(redis_connection_string)
        .await
        .expect("Failed to connect to redis");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(middleware::Logger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(store.clone(), secret_key.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .supports_credentials(),
            )
            .configure(auth::init_service)
            .configure(conversation::init_service)
            .configure(message::init_service)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
