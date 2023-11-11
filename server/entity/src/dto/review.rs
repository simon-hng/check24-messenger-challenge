use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ReviewDTO {
    pub conversation_id: Uuid,
    pub score: i32,
}
