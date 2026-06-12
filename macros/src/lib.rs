mod soft_delete;

use proc_macro::TokenStream;

/// `#[soft_delete]` 属性宏，为 SeaORM Entity 注入软删除能力
///
/// # 要求的字段名称（硬编码）
///
/// Entity 必须包含以下字段，字段名必须完全匹配：
/// - `id: i32` — 主键（用于定位记录）
/// - `delete_ts: Option<DateTime>` — 软删除时间戳（NULL 表示未删除）
/// - `create_ts: DateTime` — 创建时间（insert 时自动设置）
/// - `update_ts: DateTime` — 更新时间（insert/update 时自动设置）
///
/// # 生成的方法
///
/// ## Model 方法
/// - `delete(&self, db)` → 软删除（设置 delete_ts）
/// - `hard_delete(self, db)` → 硬删除（真正移除记录）
///
/// ## Entity 方法
/// - `find()` → 查询未删除记录（自动添加 `WHERE delete_ts IS NULL`）
/// - `find_by_id(id)` → 按 ID 查询未删除记录
/// - `find_all()` → 查询所有记录（含已删除）
///
/// ## ActiveModelBehavior
/// - `before_save()` → insert 时设置 create_ts + update_ts，update 时设置 update_ts
///
/// # 示例
///
/// ```rust,ignore
/// use sea_orm::entity::prelude::*;
/// use serde::Deserialize;
///
/// #[soft_delete]
/// #[sea_orm::model]
/// #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize)]
/// #[sea_orm(table_name = "my_table")]
/// pub struct Model {
///     #[sea_orm(primary_key)]
///     pub id: i32,
///     pub name: String,
///     pub create_ts: DateTime,
///     pub update_ts: DateTime,
///     pub delete_ts: Option<DateTime>,
/// }
/// ```
#[proc_macro_attribute]
pub fn soft_delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    soft_delete::expand(attr, item)
}
