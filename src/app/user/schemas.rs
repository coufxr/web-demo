use crate::tools::{format_date_time, format_option_date_time};
use sea_orm::prelude::DateTimeLocal;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserListInput {
    pub name: Option<String>,
    pub telephone: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Clone, Serialize, Deserialize, FromQueryResult)]
pub struct UserListOutput {
    pub id: i32,
    pub nickname: String,
    pub r#type: i8,
    pub name: Option<String>,
    pub gender: i8,
    pub telephone: Option<String>,
}

#[derive(Deserialize)]
pub struct UserInput {
    pub id: i32,
}

#[derive(Default, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct UserOutput {
    pub id: i32,
    pub uid: String,
    pub nickname: String,
    pub name: Option<String>,
    pub gender: i8,
    pub telephone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub r#type: i8,
    #[serde(serialize_with = "format_option_date_time")]
    pub last_login_at: Option<DateTimeLocal>,
    #[serde(serialize_with = "format_date_time")]
    pub created_dt: DateTimeLocal,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserCreate {
    pub nickname: String,
    pub password: String,
    pub name: Option<String>,
    pub gender: i8,
    pub telephone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
}
