use actix_identity::Identity;
use actix_web::*;
use entity::app::AppState;
use service::Mutation;

use crate::resource::auth::get_user_id;

#[post("")]
pub async fn create_review(
    data: web::Data<AppState>,
    user: Identity,
    review: web::Json<entity::dto::review::ReviewDTO>,
) -> Result<impl Responder> {
    let _user_id = get_user_id(user)?;
    let review = review.into_inner();

    let db_review = Mutation::create_review(&data.conn, review)
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    Ok(HttpResponse::Created().json(db_review))
}

pub fn init_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/review").service(create_review));
}
