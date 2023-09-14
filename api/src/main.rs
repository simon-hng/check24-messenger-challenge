use std::{
    sync::{atomic::AtomicUsize, Arc},
    time::Instant,
};

use actix::*;
use actix_cors::Cors;
use actix_web::{
    get, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use actix_web_actors::ws;
use api::{establish_connection, models::*};
use diesel::prelude::*;
use serde::Serialize;

mod server;
mod session;

#[get("/list")]
async fn list_chats() -> Result<impl Responder> {
    use api::schema::conversation::dsl::*;

    #[derive(Serialize)]
    struct ConversationInfo {
        id: i32,
        name: String,
        last_message: Option<String>,
        updated_at: chrono::NaiveDateTime,
        count_unread: i32,
    }

    let connection = &mut establish_connection();
    let results = conversation
        .select(Conversation::as_select())
        .load(connection)
        .expect("Error loading conversations");

    let conversation_info: Vec<ConversationInfo> = results
        .iter()
        .map(|conv| ConversationInfo {
            id: conv.id,
            name: conv.customer_name.to_owned(),
            updated_at: conv.updated_at,
            count_unread: 1,
            last_message: None,
        })
        .collect();

    Ok(web::Json(conversation_info))
}

#[get("/ws")]
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        session::WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // set up applications state
    // keep a count of the number of visitors
    let app_state = Arc::new(AtomicUsize::new(0));

    // start chat server actor
    let server = server::ChatServer::new(app_state.clone()).start();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(app_state.clone()))
            .app_data(web::Data::new(server.clone()))
            .wrap(middleware::Logger::default())
            .wrap(Cors::default().allowed_origin("http://localhost:5173"))
            .service(
                web::scope("/conversation")
                    .service(list_chats)
                    .service(chat_route),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
