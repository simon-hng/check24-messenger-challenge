use crate::sea_orm_active_enums::AccountType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateAccountDTO {
    pub name: String,
    pub picture: Option<String>,
    pub account_type: Option<AccountType>,
}
