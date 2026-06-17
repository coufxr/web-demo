mod rate_limit;
mod soft_delete;
mod with_transaction;

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

/// 频率限制检查属性宏
///
/// 在函数体执行前插入原子频率限制检查，已达上限则自动 `return Err(429)`。
/// 函数必须有一个名为 `redis` 的参数（`&redis::aio::ConnectionManager`）。
///
/// # Usage
/// ```rust,ignore
/// #[rate_limit_check(&format!("sms:limit:{}", phone), 60)]
/// pub async fn send_code(redis: &redis::aio::ConnectionManager, phone: &str) -> Result<String, AppError> {
///     // ...
/// }
/// ```
#[proc_macro_attribute]
pub fn rate_limit_check(attr: TokenStream, item: TokenStream) -> TokenStream {
    rate_limit::expand(attr, item)
}

/// 数据库事务属性宏
///
/// 自动在函数体执行前开启事务，执行成功后提交，失败或 panic 时回滚。
/// 函数参数中必须有一个名为 `db` 的类型为 `&sea_orm::DbConn` 的参数。
/// 函数体内的 `db` 自动被替换为事务连接。
///
/// ⚠️ **禁止在函数体中使用 `return` 语句**。`return Ok(...)` 会跳过 commit
///     导致数据丢失；`return Err(...)` 会绕过 commit 但走 Drop 回滚，行为正确
///     但不推荐。请始终用 `?` 传播错误，用尾表达式返回成功值。
///
/// # Usage
/// ```rust,ignore
/// #[with_transaction]
/// pub async fn my_fn(
///     db: &sea_orm::DbConn,
///     other: &str,
/// ) -> Result<MyModel, AppError> {
///     // 直接使用 db 进行数据库操作，会自动在事务中执行
///     let model = MyModel::find().one(db).await?;
///     // ... 其他操作 ...
///     Ok(model)
///     // 函数返回 Ok 时自动 commit，Err 时自动 rollback
/// }
/// ```
#[proc_macro_attribute]
pub fn with_transaction(attr: TokenStream, item: TokenStream) -> TokenStream {
    with_transaction::expand(attr, item)
}
