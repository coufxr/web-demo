use sea_orm::FromQueryResult;
use sea_orm::prelude::DateTimeLocal;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::apps::user::constants::{ClassType, GenderType};
use crate::helper::tools::{format_date_time, format_option_date_time};

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
pub struct UserListInput {
    pub name: Option<String>,
    pub telephone: Option<String>,
    pub r#type: Option<ClassType>,
}

#[derive(Clone, Serialize, Deserialize, FromQueryResult, ToSchema)]
pub struct UserListOutput {
    pub id: i32,
    pub nickname: String,
    pub r#type: ClassType,
    pub name: Option<String>,
    pub gender: Option<GenderType>,
    pub telephone: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, FromQueryResult)]
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

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema, Validate)]
pub struct UserCreate {
    #[validate(length(min = 1, message = "昵称不能为空"))]
    pub nickname: String,
    #[validate(length(min = 6, message = "密码长度不能少于6位"))]
    pub password: String,
    pub name: Option<String>,
    pub gender: Option<GenderType>,
    pub telephone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct UserPatch {
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub gender: Option<GenderType>,
    pub telephone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
}
