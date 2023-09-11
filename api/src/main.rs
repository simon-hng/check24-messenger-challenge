use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use api::{establish_connection, models::Chat};
use diesel::prelude::*;

#[get("/")]
async fn hello() -> impl Responder {
    use api::schema::chat::dsl::*;

    let connection = &mut establish_connection();
    let results = chat
        .limit(5)
        .select(Chat::as_select())
        .load(connection)
        .expect("Error loading chats");

    for result in results {
        println!("{}", result.title);
    }

    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
