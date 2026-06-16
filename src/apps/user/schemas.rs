use sea_orm::FromQueryResult;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use super::constants::{ClassType, GenderType};
use crate::helper::tools::{format_date_time, format_option_date_time};

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
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

#[derive(Clone, Serialize, Deserialize, FromQueryResult, ToSchema)]
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
    #[schema(value_type = Option<String>)]
    pub last_login_dt: Option<DateTime>,
    #[serde(serialize_with = "format_date_time")]
    #[schema(value_type = String)]
    pub create_ts: DateTime,
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

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema, Validate)]
pub struct UserPatch {
    pub nickname: Option<String>,
    #[validate(length(min = 6, message = "密码长度不能少于6位"))]
    pub password: Option<String>,
    pub name: Option<String>,
    pub gender: Option<GenderType>,
    pub telephone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
}
