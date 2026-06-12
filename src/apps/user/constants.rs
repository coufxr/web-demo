use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 用户类型: 1-普通用户, 2-内部员工
#[derive(
    EnumIter,
    DeriveActiveEnum,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Default,
    ToSchema,
)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
#[repr(i16)]
pub enum ClassType {
    #[default]
    #[serde(rename = "1")]
    User = 1,
    #[serde(rename = "2")]
    InternalStaff = 2,
}

/// 性别: 1-男, 2-女
#[derive(
    EnumIter,
    DeriveActiveEnum,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Default,
    ToSchema,
)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
#[repr(i16)]
pub enum GenderType {
    #[default]
    #[serde(rename = "1")]
    Male = 1,
    #[serde(rename = "2")]
    Female = 2,
}
