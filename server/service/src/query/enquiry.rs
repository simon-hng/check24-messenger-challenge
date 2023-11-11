use crate::Query;
use entity::prelude::Enquiry;
use sea_orm::prelude::*;

impl Query {
    pub async fn get_enquiries(db: &DbConn) -> Result<Vec<entity::enquiry::Model>, DbErr> {
        Enquiry::find().all(db).await
    }
}
