use actix::{Actor, StreamHandler};
use actix_cors::Cors;
use actix_web::{
    get, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use actix_web_actors::ws;
use api::{establish_connection, models::*};
use diesel::prelude::*;

struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/list")]
async fn list_chats() -> Result<impl Responder> {
    use api::schema::conversation::dsl::*;

    let connection = &mut establish_connection();
    let results = conversation
        .select(Conversation::as_select())
        .load(connection)
        .expect("Error loading conversations");

    Ok(web::Json(results))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::default().allowed_origin("http://localhost:5173"))
            .service(
                web::scope("/conversation")
                    .service(hello)
                    .service(list_chats),
            )
            .route("/ws", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
