use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::states::AppState;

use super::models::edu_account;
use super::schemas::{UserInput, UserOutput};

// Extension 扩展引入需要与main中注册的元素一致
pub async fn user_list(Extension(state): Extension<Arc<AppState>>) -> Json<Vec<UserOutput>> {
    let qs = edu_account::Entity::find()
        .filter(edu_account::Column::AccountType.eq(2))
        .all(&state.conn)
        .await;

    match qs {
        Ok(data) => {
            // 将查询结果转换为Vec<User>
            let mut result: Vec<UserOutput> = Vec::new();
            for user in data {
                result.push(UserOutput {
                    id: user.id,
                    name: user.name,
                    data: user.gender.to_string(),
                });
            }
            return Json(result);
        }
        Err(_) => todo!(),
    }
}

pub async fn user_detail(Path(input): Path<UserInput>) -> Json<UserOutput> {
    let data = UserOutput {
        id: input.id,
        name: format!("User {}", "hhhh"),
        data: "data".to_string(),
    };
    Json(data)
}
