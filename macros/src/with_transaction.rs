use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

pub fn expand(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let attrs = &func.attrs;
    let vis = &func.vis;
    let sig = &func.sig;
    let body = &func.block;

    let expanded = quote! {
        #(#attrs)*
        #vis #sig {
            use ::sea_orm::TransactionTrait;
            let __txn = db.begin().await.map_err(|e| {
                tracing::error!("事务启动失败: {}", e);
                crate::project::error::AppError::internal("服务器内部错误")
            })?;
            let __result = {
                let db = &__txn;
                #body
            };
            if __result.is_ok() {
                __txn.commit().await.map_err(|e| {
                    tracing::error!("事务提交失败: {}", e);
                    crate::project::error::AppError::internal("服务器内部错误")
                })?;
            }
            __result
        }
    };

    TokenStream::from(expanded)
}
