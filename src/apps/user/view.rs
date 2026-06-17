use super::constants::ClassType;
use super::schemas::{UserCreate, UserListInput, UserListOutput, UserOutput, UserPatch};
use crate::constants::AppState;
use crate::helper::crypto;
use crate::project::error::{ApiResult, AppError, ok};
use crate::project::extractor::ValidatedJson;
use crate::project::extractor::ValidatedQuery;
use crate::project::middlewares::auth::AuthContext;
use crate::project::pagination::{PagePagination, PaginationInput};
use axum::extract::State;
use axum::http::StatusCode;
use entity::prelude::Account;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, IntoActiveModel, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;

/// 获取用户列表
#[utoipa::path(
    get,
    path = "/user/list",
    tag = "用户管理",
    params(UserListInput,PaginationInput),
    responses(
        (status = 200, body = PagePagination<UserListOutput>)
    )
)]
pub async fn user_list(
    State(state): State<AppState>,
    ValidatedQuery(input): ValidatedQuery<UserListInput>,
    ValidatedQuery(pagination): ValidatedQuery<PaginationInput>,
) -> ApiResult<PagePagination<UserListOutput>> {
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
    path = "/user",
    tag = "用户管理",
    request_body = UserCreate,
    responses(
        (status = 200),
        (status = 400, description = "Validation error")
    )
)]
pub async fn user_create(
    State(state): State<AppState>,
    ValidatedJson(input): ValidatedJson<UserCreate>,
) -> ApiResult<()> {
    // 密码哈希
    let hashed_password = crypto::hash_password(&input.password)?;

    let obj = Account::ActiveModel {
        uid: Set(Uuid::new_v4().to_string()),
        nickname: Set(input.nickname),
        password: Set(hashed_password),
        name: Set(input.name),
        gender: Set(input.gender.map(|t| t as i16).unwrap_or(0)),
        telephone: Set(input.telephone),
        email: Set(input.email),
        address: Set(input.address),
        r#type: Set(ClassType::User as i16),
        ..Default::default()
    };

    let _obj = obj.insert(&state.db).await?;

    ok(())
}

/// 获取当前用户信息
#[utoipa::path(
    get,
    path = "/user",
    tag = "用户管理",
    responses(
        (status = 200, body = UserOutput),
        (status = 404, description = "User not found")
    )
)]
pub async fn user_detail(
    State(state): State<AppState>,
    auth: AuthContext,
) -> ApiResult<UserOutput> {
    let user = Account::Entity::find_by_id(auth.user_id)
        .into_model::<UserOutput>()
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::Api(StatusCode::NOT_FOUND, "用户不存在".to_string()))?;
    ok(user)
}

/// 更新当前用户信息
#[utoipa::path(
    patch,
    path = "/user",
    tag = "用户管理",
    request_body = UserPatch,
    responses(
        (status = 200),
        (status = 404, description = "User not found")
    )
)]
pub async fn user_patch(
    State(state): State<AppState>,
    auth: AuthContext,
    ValidatedJson(data): ValidatedJson<UserPatch>,
) -> ApiResult<()> {
    let mut obj = Account::Entity::find_by_id(auth.user_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::Api(StatusCode::NOT_FOUND, "用户不存在".to_string()))?
        .into_active_model();

    if let Some(v) = data.nickname {
        obj.nickname = Set(v)
    }
    if let Some(v) = data.password {
        let hashed_password = crypto::hash_password(&v)?;
        obj.password = Set(hashed_password)
    }
    if let Some(v) = data.gender {
        obj.gender = Set(v as i16)
    }
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

    obj.update(&state.db).await?;
    ok(())
}

/// 删除当前用户
#[utoipa::path(
    delete,
    path = "/user",
    tag = "用户管理",
    responses(
        (status = 200),
        (status = 404, description = "User not found")
    )
)]
pub async fn user_delete(State(state): State<AppState>, auth: AuthContext) -> ApiResult<()> {
    let obj = Account::Entity::find_by_id(auth.user_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| AppError::Api(StatusCode::NOT_FOUND, "用户不存在".to_string()))?;
    obj.delete(&state.db).await?;

    ok(())
}
