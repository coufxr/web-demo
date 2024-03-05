//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.12

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "account")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub uid: String,
    pub r#type: i8,
    pub nickname: String,
    pub password: String,
    pub name: Option<String>,
    pub gender: Option<i8>,
    pub telephone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub last_login_at: Option<DateTimeLocal>,
    pub created_dt: DateTimeLocal,
    pub updated_dt: DateTimeLocal,
    pub deleted_dt: Option<DateTimeLocal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}