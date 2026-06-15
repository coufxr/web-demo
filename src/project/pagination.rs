use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

/// 分页查询参数
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema, IntoParams, Validate, Default)]
#[into_params(parameter_in = Query)]
pub struct PaginationInput {
    /// 页码（从 1 开始）
    #[serde(default = "default_page")]
    #[param(default = 1, example = 1, minimum = 1)]
    #[validate(range(min = 1))]
    pub page: u64,
    /// 每页条数
    #[serde(default = "default_page_size")]
    #[param(default = 10, example = 10, minimum = 5, maximum = 100)]
    #[validate(range(min = 5, max = 100))]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}

fn default_page_size() -> u64 {
    10
}

/// 分页响应结构
#[derive(Serialize, ToSchema)]
pub struct PagePagination<T: ToSchema> {
    /// 数据列表
    pub data: Vec<T>,
    /// 总条数
    #[schema(example = 100, minimum = 0)]
    pub total: u64,
    /// 当前页码
    #[schema(example = 1, minimum = 1)]
    pub page: u64,
    /// 每页条数
    #[schema(example = 10, minimum = 1)]
    pub page_size: u64,
}

/// 偏移量分页响应结构
#[allow(dead_code)]
#[derive(Serialize, ToSchema)]
pub struct OffsetPagination<T: ToSchema> {
    /// 数据列表
    pub data: Vec<T>,
    /// 总条数
    #[schema(example = 100, minimum = 0)]
    pub total: u64,
    /// 偏移量
    #[schema(example = 0, minimum = 0)]
    pub offset: u64,
    /// 每页条数
    #[schema(example = 10, minimum = 1)]
    pub limit: u64,
}

/// 游标分页响应结构
#[allow(dead_code)]
#[derive(Serialize, ToSchema)]
pub struct CursorPagination<T: ToSchema> {
    /// 数据列表
    pub data: Vec<T>,
    /// 下一页游标
    #[schema(example = "abc123")]
    pub next_cursor: Option<String>,
    /// 是否还有更多数据
    #[schema(example = true)]
    pub has_more: bool,
}
