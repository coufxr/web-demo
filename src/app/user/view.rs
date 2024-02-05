use axum::{extract::Path, Json};

use super::schemas::{UserInput, UserOutput};

pub async fn user_list() -> Json<Vec<UserOutput>> {
    let mut data: Vec<UserOutput> = Vec::new();
    for i in 0..10 {
        data.push(UserOutput {
            id: i,
            name: format!("name {i}"),
            data: format!("data {i}"),
        });
    }
    return Json(data);
}

pub async fn user_detail(Path(input): Path<UserInput>) -> Json<UserOutput> {
    let data = UserOutput {
        id: input.id,
        name: format!("User {}", "hhhh"),
        data: "data".to_string(),
    };
    Json(data)
}
