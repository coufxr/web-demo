use sea_orm::FromQueryResult;
use sea_orm::prelude::DateTimeLocal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::apps::user::constants::{ClassType, GenderType};
use crate::helper::tools::{format_date_time, format_option_date_time};

#[derive(Debug, Deserialize, Validate)]
pub struct UserListInput {
    pub name: Option<String>,
    pub telephone: Option<String>,
    pub r#type: Option<ClassType>,
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    #[validate(range(min = 5, max = 100))]
    pub page_size: Option<u64>,
}

#[derive(Clone, Serialize, Deserialize, FromQueryResult)]
pub struct UserListOutput {
    pub id: i32,
    pub nickname: String,
    pub r#type: ClassType,
    pub name: Option<String>,
    pub gender: Option<GenderType>,
    pub telephone: Option<String>,
}

#[derive(Default, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct UserOutput {
    pub id: i32,
    pub uid: String,
    pub nickname: String,
    pub name: Option<String>,
    pub gender: Option<GenderType>,
    pub telephone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub r#type: ClassType,
    #[serde(serialize_with = "format_option_date_time")]
    pub last_login_dt: Option<DateTimeLocal>,
    #[serde(serialize_with = "format_date_time")]
    pub create_ts: DateTimeLocal,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserCreate {
    pub nickname: String,
    pub password: String,
    pub name: Option<String>,
    pub gender: Option<GenderType>,
    pub telephone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserPatch {
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub gender: Option<GenderType>,
    pub telephone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
}
