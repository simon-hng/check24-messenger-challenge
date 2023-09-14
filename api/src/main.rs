use std::sync::{atomic::AtomicUsize, Arc};

use actix::Actor;
use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};

use api::handler::{account, conversation};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // set up applications state
    // keep a count of the number of visitors
    let app_state = Arc::new(AtomicUsize::new(0));

    // start chat server actor
    let server = api::handler::server::ChatServer::new(app_state.clone()).start();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(app_state.clone()))
            .app_data(web::Data::new(server.clone()))
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
