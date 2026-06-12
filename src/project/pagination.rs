use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema, IntoParams, Validate, Default)]
pub struct PaginationInput {
    #[serde(default = "default_page")]
    #[param(default = 1, example = 1)]
    #[validate(range(min = 1))]
    pub page: u64,
    #[serde(default = "default_page_size")]
    #[param(default = 10, example = 10)]
    #[validate(range(min = 5, max = 100))]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}

fn default_page_size() -> u64 {
    10
}

#[derive(Serialize, ToSchema)]
pub struct PagePagination<T: ToSchema> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

#[allow(dead_code)]
#[derive(Serialize, ToSchema)]
pub struct OffsetPagination<T: ToSchema> {
    pub data: Vec<T>,
    pub total: u64,
    pub offset: u64,
    pub limit: u64,
}

#[allow(dead_code)]
#[derive(Serialize, ToSchema)]
pub struct CursorPagination<T: ToSchema> {
    pub data: Vec<T>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}
