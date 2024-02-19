use std::sync::Arc;

use axum::{
    extract::{Json, Path, Query},
    Extension,
};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::response::{EmptyStruct, JsonResponse};
use crate::states::AppState;

use super::models::prelude::Account;
use super::schemas::{UserCreate, UserInput, UserListInput, UserListOutput, UserOutput};

// Extension 扩展引入需要与main中注册的元素一致
pub async fn user_list(
    Extension(state): Extension<Arc<AppState>>,
    Query(input): Query<UserListInput>,
) -> AppResult<Vec<UserListOutput>> {
    let data = Account::Entity::find()
        .select_only() // 指定加载哪些字段
        .columns([
            Account::Column::Id,
            Account::Column::Nickname,
            Account::Column::Type,
            Account::Column::Name,
            Account::Column::Gender,
            Account::Column::Telephone,
        ])
        .filter(
            // 实现 字段存在及查询. 不存在则跳过
            Condition::all()
                .add_option(Some(Account::Column::Type.eq(2)))
                .add_option(input.name.map(|n| Account::Column::Name.contains(n)))
                .add_option(
                    input
                        .telephone
                        .map(|t| Account::Column::Telephone.contains(t)),
                ),
        )
        .order_by_desc(Account::Column::Id) //排序
        .into_model::<UserListOutput>() //指定的字段需要在此处进行接收, 否则原本 model 会因为字段缺失而报错
        // .all(&state.db) // 获取全部的数据
        .paginate(&state.db, input.page_size.unwrap_or(10))
        .fetch_page(input.page.unwrap_or(1) - 1) //page 页数从 `0` 开始算起
        .await
        .map_err(AppError::from)?;

    Ok(JsonResponse::success(data))
}

pub async fn user_create(
    Extension(state): Extension<Arc<AppState>>,
    Json(input): Json<UserCreate>,
) -> AppResult<EmptyStruct> {
    let obj = Account::ActiveModel {
        uid: Set(Uuid::new_v4().to_string()),
        nickname: Set(input.nickname),
        password: Set(input.password),
        name: Set(input.name),
        gender: Set(input.gender),
        telephone: Set(input.telephone),
        email: Set(input.email),
        address: Set(input.address),
        r#type: Set(1),
        ..Default::default()
    };

    Account::Entity::insert(obj)
        .exec(&state.db)
        .await
        .map_err(AppError::from)?;

    Ok(JsonResponse::success(EmptyStruct::default()))
}

pub async fn user_detail(
    Extension(state): Extension<Arc<AppState>>,
    Path(input): Path<UserInput>,
) -> AppResult<UserOutput> {
    let qs = Account::Entity::find_by_id(input.id)
        .into_model::<UserOutput>()
        .one(&state.db)
        .await
        .map_err(AppError::from)?;
    // .unwrap_or_default(); // 可直接使用model的默认值

    if qs.is_none() {
        return Err(AppError::Other("未找到对应数据".to_string()));
    }

    Ok(JsonResponse::success(qs.unwrap()))
}
