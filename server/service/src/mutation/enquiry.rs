use crate::Mutation;
use sea_orm::prelude::*;
use sea_orm::Set;

impl Mutation {
    pub async fn create_enquiry(
        db: &DbConn,
        enquiry: entity::dto::enquiry::EnquiryDTO,
    ) -> Result<entity::enquiry::Model, DbErr> {
        entity::enquiry::ActiveModel {
            enquirer_id: Set(enquiry.enquirer_id),
            title: Set(enquiry.title),
            description: Set(enquiry.description),
            ..Default::default()
        }
        .insert(db)
        .await
    }
}
