use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserInput {
    pub id: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserOutput {
    pub id: i32,
    pub name: String,
    pub data: String,
}
