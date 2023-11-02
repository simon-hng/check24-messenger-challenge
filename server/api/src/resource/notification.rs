use actix::Addr;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use service::chat::{server, session};
use std::time::Instant;

#[get("/ws")]
async fn receive_notifications(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<server::NotificationServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        session::WsNotifierSession {
            heart_beat: Instant::now(),
            addr: server.get_ref().clone(),
            account_id: None,
        },
        &req,
        stream,
    )
}

pub fn init_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/notifications").service(receive_notifications));
}
