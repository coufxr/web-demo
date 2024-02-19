use std::sync::Arc;

use axum::extract::Query;
use axum::{extract::Path, Extension};
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};

use crate::error::{AppError, AppResult};
use crate::response::JsonResponse;
use crate::states::AppState;

use super::models::prelude::Account;
use super::schemas::{UserInput, UserListInput, UserListOutput, UserOutput};

// Extension 扩展引入需要与main中注册的元素一致
pub async fn user_list(
    Query(input): Query<UserListInput>,
    Extension(state): Extension<Arc<AppState>>,
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
        // .all(&state.conn) // 获取全部的数据
        .paginate(&state.conn, input.page_size.unwrap_or(10))
        .fetch_page(input.page.unwrap_or(1) - 1) //page 页数从 `0` 开始算起
        .await
        .map_err(AppError::from)?;

    Ok(JsonResponse::success(data))
}

pub async fn user_detail(
    Path(input): Path<UserInput>,
    Extension(state): Extension<Arc<AppState>>,
) -> AppResult<UserOutput> {
    let qs = Account::Entity::find_by_id(input.id)
        .into_model::<UserOutput>()
        .one(&state.conn)
        .await
        .map_err(AppError::from)?;
    // .unwrap_or_default(); // 可直接使用model的默认值

    if qs.is_none() {
        return Err(AppError::Other("未找到对应数据".to_string()));
    }

    Ok(JsonResponse::success(qs.unwrap()))
}
