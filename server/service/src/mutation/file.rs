use crate::Mutation;
use base64::{engine::general_purpose, Engine as _};
use entity::prelude::File;
use sea_orm::prelude::*;
use sea_orm::InsertResult;
use sea_orm::Set;

impl Mutation {
    pub async fn create_message_attachments(
        db: &DatabaseConnection,
        message_id: Uuid,
        base64_objects: Vec<String>,
    ) -> Result<InsertResult<entity::file::ActiveModel>, sea_orm::DbErr> {
        let engine = general_purpose::STANDARD_NO_PAD;

        let decoded: Vec<entity::file::ActiveModel> = base64_objects
            .iter()
            .map(|obj| {
                let object = engine.decode(obj).unwrap();
                entity::file::ActiveModel {
                    object: Set(object),
                    message_id: Set(Some(message_id)),
                    ..Default::default()
                }
            })
            .collect();

        File::insert_many(decoded).exec(db).await
    }
}
