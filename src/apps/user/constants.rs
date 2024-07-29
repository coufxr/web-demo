use sea_orm::{DeriveActiveEnum, EnumIter};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr,
)]
#[sea_orm(rs_type = "i8", db_type = "Integer")]
#[repr(i8)]
pub enum ClassType {
    User = 1,
    InternalStaff = 2,
}

#[derive(
    EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr,
)]
#[sea_orm(rs_type = "i8", db_type = "Integer")]
#[repr(i8)]
pub enum GenderType {
    Male = 1,
    Female = 2,
}
