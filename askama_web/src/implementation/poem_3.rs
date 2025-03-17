pub use std::marker::Send;

pub use askama::Template;
use http_1::StatusCode;
use http_1::header::{CONTENT_TYPE, HeaderValue};
pub use poem_3::{IntoResponse, Response};

pub use crate::__askama_web_impl_poem_3 as derive;

#[macro_export]
#[doc(hidden)]
macro_rules! __askama_web_impl_poem_3 {
    (@ $ast:tt) => {
        $crate::__askama_web_impl::askama_web_derive::impl_framework! {
            $crate::__askama_web_impl::poem_3::derive!(
                $ast
                where
                    Self: $crate::__askama_web_impl::poem_3::Send
            );
        }
    };
    (
        ident: [$ident:ident],
        impl_generics: $_impl_generics:tt,
        ty_generics: $_ty_generics:tt,
        where_clause: $_where_clause:tt,
        ex_impl_generics: [$($impl_generics:tt)*],
        ex_ty_generics: [$($ty_generics:tt)*],
        ex_where_clause: [$($where_clause:tt)*],
    ) => {
        const _: () = {
            use $crate::__askama_web_impl::poem_3 as __askama_web;

            impl $($impl_generics)* __askama_web::IntoResponse
            for $ident $($ty_generics)* $($where_clause)* {
                #[inline]
                #[track_caller]
                fn into_response(self) -> $crate::__askama_web_impl::poem_3::Response {
                    let result = <Self as __askama_web::Template>::render(&self);
                    __askama_web::into_response(result)
                }
            }
        };
    };
}

#[track_caller]
pub fn into_response(result: askama::Result<String>) -> Response {
    match result {
        Ok(body) => Response::builder()
            .header(
                CONTENT_TYPE,
                HeaderValue::from_static("text/html; charset=utf-8"),
            )
            .status(StatusCode::OK)
            .body(body),
        Err(err) => {
            crate::render_error(&err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
