use actix_identity::Identity;
use actix_web::*;
use entity::app::AppState;
use service::Mutation;

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

pub fn init_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/enquiry").service(create_enquiry));
}
