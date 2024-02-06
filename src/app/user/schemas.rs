use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserListInput {
    pub name: Option<String>,
    pub telephone: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct UserListOutput {
    pub id: i32,
    pub uid: String,
    pub account_name: Option<String>,
    pub name: String,
    pub gender: i8,
}

#[derive(Serialize, Deserialize)]
pub struct UserInput {
    pub id: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserOutput {
    pub id: i32,
    pub uid: String,
    pub account_name: Option<String>,
    pub name: String,
    pub gender: i8,
    pub telephone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub account_type: i8,
    pub last_login_at: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
}
