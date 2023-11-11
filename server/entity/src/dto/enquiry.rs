use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EnquiryDTO {
    pub enquirer_id: Uuid,
    pub title: String,
    pub description: String,
}
