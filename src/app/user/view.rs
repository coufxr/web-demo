use std::sync::Arc;

use anyhow::anyhow;
use axum::extract::Query;
use axum::{extract::Path, Extension};
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};

use crate::error::AppError;
use crate::response::JsonResponse;
use crate::states::AppState;

use super::models::edu_account;
use super::schemas::{UserInput, UserListInput, UserListOutput, UserOutput};

// Extension 扩展引入需要与main中注册的元素一致
pub async fn user_list(
    Query(input): Query<UserListInput>,
    Extension(state): Extension<Arc<AppState>>,
) -> JsonResponse<Vec<UserListOutput>> {
    let qs = edu_account::Entity::find()
        .select_only() // 指定加载哪些字段
        .columns([
            edu_account::Column::Id,
            edu_account::Column::AccountName,
            edu_account::Column::AccountType,
            edu_account::Column::Name,
            edu_account::Column::Gender,
            edu_account::Column::Telephone,
        ])
        .filter(
            // 实现 字段存在及查询. 不存在则跳过
            Condition::all()
                .add_option(Some(edu_account::Column::AccountType.eq(2)))
                .add_option(input.name.map(|n| edu_account::Column::Name.contains(n)))
                .add_option(
                    input
                        .telephone
                        .map(|t| edu_account::Column::Gender.contains(t)),
                ),
        )
        .order_by_desc(edu_account::Column::Id) //排序
        .into_model::<UserListOutput>() //指定的字段需要在此处进行接收, 否则原本 model 会因为字段缺失而报错
        // .all(&state.conn) // 获取全部的数据
        .paginate(&state.conn, input.page_size.unwrap_or(10))
        .fetch_page(input.page.unwrap_or(1) - 1) //page 页数从 `0` 开始算起
        .await;

    match qs {
        Ok(data) => JsonResponse::success(data),
        Err(err) => {
            eprintln!("{err}");
            todo!()
        }
    }
}

pub async fn user_detail(
    Path(input): Path<UserInput>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<JsonResponse<UserOutput>, AppError> {
    let qs = edu_account::Entity::find_by_id(input.id)
        .into_model::<UserOutput>()
        .one(&state.conn)
        .await
        .map_err(AppError::from)?;
    // .unwrap_or_default(); // 可直接使用model的默认值

    if qs.is_none() {
        return Err(anyhow!("nill").into());
    }

    Ok(JsonResponse::success(qs.unwrap()))
}
