use std::sync::{atomic::AtomicUsize, Arc};

use actix::Actor;
use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};

use api::handler::{account, conversation};

use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::env;

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenv().ok();

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
            .wrap(Cors::default().allowed_origin("http://localhost:5173"))
            .service(
                web::scope("/conversation")
                    .service(conversation::list_chats)
                    .service(conversation::chat_route),
            )
            .service(
                web::scope("/account")
                    .service(account::list_accounts)
                    .service(account::get_account_by_id),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
