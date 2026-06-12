use super::schemas::{UserCreate, UserListInput, UserListOutput, UserOutput, UserPatch};
use crate::apps::user::constants::ClassType;
use crate::constants::AppState;
use crate::project::error::{ApiResult, AppError, ok};
use crate::project::extractor::ResourceId;
use crate::project::pagination::{PagePagination, PaginationInput};
use axum::extract::State;
use axum::extract::{Json, Query};
use axum::http::StatusCode;
use entity::prelude::Account;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, IntoActiveModel, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;
use validator::Validate;

/// 获取用户列表
#[utoipa::path(
    get,
    path = "",
    tag = "用户管理",
    params(UserListInput,PaginationInput),
    responses(
        (status = 200, body = PagePagination<UserListOutput>)
    )
)]
pub async fn user_list(
    State(state): State<AppState>,
    Query(input): Query<UserListInput>,
    Query(pagination): Query<PaginationInput>,
) -> ApiResult<PagePagination<UserListOutput>> {
    input.validate()?;

    let page = pagination.page;
    let page_size = pagination.page_size;

    let query = Account::Entity::find()
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
        );

    let total = query.clone().count(&state.db).await?;
    let data = query
        .order_by_desc(Account::Column::Id)
        .into_model::<UserListOutput>()
        .paginate(&state.db, page_size)
        .fetch_page(page - 1)
        .await?;

    ok(PagePagination {
        data,
        total,
        page,
        page_size,
    })
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
    State(state): State<AppState>,
    Json(input): Json<UserCreate>,
) -> ApiResult<()> {
    input.validate()?;
    let obj = Account::ActiveModel {
        uid: Set(Uuid::new_v4().to_string()),
        nickname: Set(input.nickname),
        password: Set(input.password),
        name: Set(input.name),
        gender: Set(input.gender.map(|t| t as i16).unwrap_or(0)),
        telephone: Set(input.telephone),
        email: Set(input.email),
        address: Set(input.address),
        r#type: Set(ClassType::User as i16),
        ..Default::default()
    };

    let _obj: Account::Model = obj.insert(&state.db).await?;

    ok(())
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
    State(state): State<AppState>,
    ResourceId(id): ResourceId,
) -> ApiResult<UserOutput> {
    let user = Account::Entity::find_by_id(id)
        .into_model::<UserOutput>()
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::Api(StatusCode::NOT_FOUND, "用户不存在".to_string()))?;
    ok(user)
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
    State(state): State<AppState>,
    ResourceId(id): ResourceId,
    Json(data): Json<UserPatch>,
) -> ApiResult<()> {
    let mut obj = Account::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::Api(StatusCode::NOT_FOUND, "用户不存在".to_string()))?
        .into_active_model();

    // 非空字段需要解包 Option
    if let Some(v) = data.nickname {
        obj.nickname = Set(v)
    }
    if let Some(v) = data.password {
        obj.password = Set(v)
    }
    if let Some(v) = data.gender {
        obj.gender = Set(v as i16)
    }
    // 可空字段直接传 Option
    if data.name.is_some() {
        obj.name = Set(data.name)
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

    let _obj = obj.update(&state.db).await?;

    ok(())
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
    State(state): State<AppState>,
    ResourceId(id): ResourceId,
) -> ApiResult<()> {
    let obj = Account::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::Api(StatusCode::NOT_FOUND, "用户不存在".to_string()))?;
    obj.delete(&state.db).await?;

    ok(())
}
