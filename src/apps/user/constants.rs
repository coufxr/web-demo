use sea_orm::{DeriveActiveEnum, EnumIter};
use serde_repr::{Deserialize_repr, Serialize_repr};
use utoipa::ToSchema;

#[derive(
    EnumIter,
    DeriveActiveEnum,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize_repr,
    Deserialize_repr,
    Default,
    ToSchema,
)]
#[sea_orm(rs_type = "u8", db_type = "Integer")]
#[repr(u8)]
pub enum ClassType {
    #[default]
    User = 1,
    InternalStaff = 2,
}

#[derive(
    EnumIter,
    DeriveActiveEnum,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize_repr,
    Deserialize_repr,
    Default,
    ToSchema,
)]
#[sea_orm(rs_type = "u8", db_type = "Integer")]
#[repr(u8)]
pub enum GenderType {
    #[default]
    Male = 1,
    Female = 2,
}
