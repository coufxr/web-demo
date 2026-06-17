use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, ItemFn, Token, parse_macro_input};

struct RateLimitAttr {
    key: Expr,
    ttl: Expr,
}

impl Parse for RateLimitAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(RateLimitAttr {
            key: input.parse()?,
            ttl: {
                input.parse::<Token![,]>()?;
                input.parse()?
            },
        })
    }
}

pub fn expand(attr: TokenStream, item: TokenStream) -> TokenStream {
    let RateLimitAttr { key, ttl } = parse_macro_input!(attr as RateLimitAttr);
    let func = parse_macro_input!(item as ItemFn);
    let attrs = &func.attrs;
    let vis = &func.vis;
    let sig = &func.sig;
    let body = &func.block;

    let expanded = quote! {
        #(#attrs)*
        #vis #sig {
            if !crate::project::redis::set_nx_with_expire(redis, #key, #ttl).await? {
                return Err(crate::project::error::AppError::Api(
                    ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                    "发送过于频繁，请稍后再试".to_string(),
                ));
            }
            #body
        }
    };

    TokenStream::from(expanded)
}
