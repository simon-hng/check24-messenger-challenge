use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware, web, App, HttpServer};
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::env;

mod resource;
use resource::*;

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

    // set up applications state
    // keep a count of the number of visitors
    let app_state = AppState { conn };

    // let server = api::handler::server::ChatServer::new(app_state.clone()).start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            // .app_data(web::Data::new(server.clone()))
            .wrap(middleware::Logger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .wrap(Cors::default().allowed_origin("http://localhost:5173"))
            .service(
                web::scope("/conversation")
                    .service(conversation::list_chats)
                    .service(conversation::chat_route),
            )
            .service(
                web::scope("/account")
                    .service(account::list_accounts)
                    .service(account::who_am_i)
                    .service(account::login)
                    .service(account::get_account_by_id),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
