use super::schemas::{UserCreate, UserListInput, UserListOutput, UserOutput, UserPatch};
use crate::apps::user::constants::ClassType;
use crate::project::error::{AppError, AppResult};
use axum::{
    Extension,
    extract::{Json, Path, Query},
};
use entity::prelude::Account;
use sea_orm::DbConn;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, EntityTrait, IntoActiveModel,
    ModelTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;
use validator::Validate;

/// 获取用户列表
#[utoipa::path(
    get,
    path = "",
    tag = "用户管理",
    responses(
        (status = 200, body = Vec<UserListOutput>)
    )
)]
pub async fn user_list(
    Extension(db): Extension<DbConn>,
    Query(input): Query<UserListInput>,
) -> AppResult<Json<Vec<UserListOutput>>> {
    input.validate()?;

    let data = Account::Entity::find()
        .select_only()
        .columns([
            Account::Column::Id,
            Account::Column::Nickname,
            Account::Column::Type,
            Account::Column::Name,
            Account::Column::Gender,
            Account::Column::Telephone,
        ])
        .filter(
            Condition::all()
                .add_option(input.r#type.map(|t| Account::Column::Type.eq(t)))
                .add_option(input.name.map(|n| Account::Column::Name.contains(n)))
                .add_option(
                    input
                        .telephone
                        .map(|t| Account::Column::Telephone.contains(t)),
                ),
        )
        .order_by_desc(Account::Column::Id)
        .into_model::<UserListOutput>()
        .paginate(&db, input.page_size.unwrap_or(10))
        .fetch_page(input.page.unwrap_or(1) - 1)
        .await
        .map_err(AppError::from)?;

    Ok(Json(data))
}

/// 创建用户
#[utoipa::path(
    post,
    path = "",
    tag = "用户管理",
    request_body = UserCreate,
    responses(
        (status = 200),
        (status = 400, description = "Validation error")
    )
)]
pub async fn user_create(
    Extension(db): Extension<DbConn>,
    Json(input): Json<UserCreate>,
) -> AppResult<Json<()>> {
    let obj = Account::ActiveModel {
        uid: Set(Uuid::new_v4().to_string()),
        nickname: Set(input.nickname),
        password: Set(input.password),
        name: Set(input.name),
        gender: Set(input.gender.map(|t| t as u8).unwrap_or(0)),
        telephone: Set(input.telephone),
        email: Set(input.email),
        address: Set(input.address),
        r#type: Set(ClassType::User as u8),
        ..Default::default()
    };

    let _obj: Account::Model = obj.insert(&db).await.map_err(AppError::from)?;

    Ok(Json(()))
}

/// 获取用户详情
#[utoipa::path(
    get,
    path = "/{id}",
    tag = "用户管理",
    responses(
        (status = 200),
        (status = 404, description = "User not found")
    )
)]
pub async fn user_detail(
    Extension(db): Extension<DbConn>,
    Path(id): Path<u32>,
) -> AppResult<Json<UserOutput>> {
    let qs = Account::Entity::find_by_id(id as i32)
        .into_model::<UserOutput>()
        .one(&db)
        .await
        .map_err(AppError::from)?;

    if qs.is_none() {
        return Err(AppError::Other("未找到对应数据".to_string()));
    }

    Ok(Json(qs.unwrap()))
}

/// 更新用户
#[utoipa::path(
    patch,
    path = "/{id}",
    tag = "用户管理",
    request_body = UserPatch,
    responses(
        (status = 200),
        (status = 404, description = "User not found")
    )
)]
pub async fn user_patch(
    Extension(db): Extension<DbConn>,
    Path(id): Path<u32>,
    Json(data): Json<UserPatch>,
) -> AppResult<Json<()>> {
    let mut obj = Account::Entity::find_by_id(id as i32)
        .one(&db)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::Other("not found".into()))?
        .into_active_model();

    if let Some(nickname) = data.nickname {
        obj.nickname = Set(nickname)
    }
    if let Some(password) = data.password {
        obj.password = Set(password)
    }
    if data.name.is_some() {
        obj.name = Set(data.name)
    }
    if data.gender.is_some() {
        obj.gender = Set(data.gender.map(|t| t as u8).unwrap_or(0));
    }
    if data.telephone.is_some() {
        obj.telephone = Set(data.telephone)
    }
    if data.email.is_some() {
        obj.email = Set(data.email)
    }
    if data.address.is_some() {
        obj.address = Set(data.address)
    }

    let _obj = obj.update(&db).await.map_err(AppError::from)?;

    Ok(Json(()))
}

/// 删除用户
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "用户管理",
    responses(
        (status = 200),
        (status = 404, description = "User not found")
    )
)]
pub async fn user_delete(
    Extension(db): Extension<DbConn>,
    Path(id): Path<u32>,
) -> AppResult<Json<()>> {
    let obj = Account::Entity::find_by_id(id as i32)
        .one(&db)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::Other("not found".into()))?;

    obj.delete(&db).await.map_err(AppError::from)?;

    Ok(Json(()))
}
