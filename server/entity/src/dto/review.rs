use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ReviewDTO {
    pub reviewer_id: Uuid,
    pub recipient_id: Uuid,
    pub score: i32,
}
