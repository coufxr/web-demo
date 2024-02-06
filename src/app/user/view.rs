use std::sync::Arc;

use axum::extract::Query;
use axum::{extract::Path, Extension};
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

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
        // .select_only() // 指定加载哪些字段  //暂时不可使用 原因未知
        // .columns([
        //     edu_account::Column::Id,
        //     edu_account::Column::Uid,
        //     edu_account::Column::AccountName,
        //     edu_account::Column::Name,
        //     edu_account::Column::Gender,
        // ])
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
        // .all(&state.conn) // 获取全部的数据
        .paginate(&state.conn, input.page_size.unwrap_or(10))
        .fetch_page(input.page.unwrap_or(1) - 1) //page 页数从 `0` 开始算起
        .await;

    match qs {
        Ok(data) => {
            // 将查询结果转换为Vec<User>
            let mut result: Vec<UserListOutput> = Vec::new();
            for user in data {
                result.push(UserListOutput {
                    id: user.id,
                    uid: user.uid,
                    account_name: user.account_name,
                    name: user.name,
                    gender: user.gender,
                });
            }
            JsonResponse::success(result)
        }
        Err(err) => {
            eprintln!("{err}");
            todo!()
        }
    }
}

pub async fn user_detail(Path(input): Path<UserInput>) -> JsonResponse<UserOutput> {
    let data = UserOutput {
        id: input.id,
        name: format!("User {}", "hhhh"),
        data: "data".to_string(),
    };

    JsonResponse::success(data)
}
