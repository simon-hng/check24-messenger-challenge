use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageQueryParams {
    pub limit: Option<u64>,
    pub before: Option<chrono::NaiveDateTime>,
}
