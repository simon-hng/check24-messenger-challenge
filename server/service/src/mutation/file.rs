use crate::Mutation;
use entity::prelude::File;
use sea_orm::prelude::*;
use sea_orm::InsertResult;
use sea_orm::Set;

impl Mutation {
    pub async fn create_message_attachments(
        db: &DatabaseConnection,
        message_id: Uuid,
        data_urls: Vec<String>,
    ) -> Result<InsertResult<entity::file::ActiveModel>, DbErr> {
        let mut decoded: Vec<entity::file::ActiveModel> = Vec::new();
        for data_url in data_urls.iter() {
            let obj = entity::file::ActiveModel {
                object: Set(data_url.to_string()),
                message_id: Set(Some(message_id)),
                ..Default::default()
            };

            decoded.push(obj);
        }

        File::insert_many(decoded).exec(db).await
    }
}
