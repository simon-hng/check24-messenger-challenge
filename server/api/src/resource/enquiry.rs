use actix_identity::Identity;
use actix_web::*;
use entity::app::AppState;
use service::{Mutation, Query};

use crate::resource::auth::get_user_id;

#[post("")]
pub async fn create_enquiry(
    data: web::Data<AppState>,
    user: Identity,
    enquiry: web::Json<entity::dto::enquiry::EnquiryDTO>,
) -> Result<impl Responder> {
    let _user_id = get_user_id(user)?;
    let enquiry = enquiry.into_inner();

    let db_enquiry = Mutation::create_enquiry(&data.conn, enquiry)
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    Ok(HttpResponse::Created().json(db_enquiry))
}

#[get("")]
pub async fn get_enquiries(data: web::Data<AppState>, user: Identity) -> Result<impl Responder> {
    let _user_id = get_user_id(user)?;
    let db_enquiries = Query::get_enquiries(&data.conn)
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    Ok(web::Json(db_enquiries))
}

pub fn init_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/enquiry")
            .service(create_enquiry)
            .service(get_enquiries),
    );
}
