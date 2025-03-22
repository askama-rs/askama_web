pub use askama::Template;
pub use axum_core_0_5::response::{IntoResponse, Response};
use http_1::StatusCode;
use http_1::header::{CONTENT_TYPE, HeaderValue};

#[cfg(feature = "derive")]
pub use crate::__askama_web_impl_axum_core_0_5 as derive;

#[cfg(feature = "derive")]
#[macro_export]
#[doc(hidden)]
macro_rules! __askama_web_impl_axum_core_0_5 {
    (@ $ast:tt) => {
        $crate::__askama_web_impl::askama_web_derive::impl_framework! {
            $crate::__askama_web_impl::axum_core_0_5::derive!($ast);
        }
    };
    (
        ident: [$ident:ident],
        impl_generics: [$($impl_generics:tt)*],
        ty_generics: [$($ty_generics:tt)*],
        where_clause: [$($where_clause:tt)*],
    ) => {
        const _: () = {
            use $crate::__askama_web_impl::axum_core_0_5 as __askama_web;

            impl $($impl_generics)* __askama_web::IntoResponse
            for $ident $($ty_generics)* $($where_clause)* {
                #[inline]
                #[track_caller]
                fn into_response(self) -> $crate::__askama_web_impl::axum_core_0_5::Response {
                    let result = <Self as __askama_web::Template>::render(&self);
                    __askama_web::into_response(result)
                }
            }
        };
    };
}

impl<T: Template> IntoResponse for crate::WebResult<T> {
    #[inline]
    #[track_caller]
    fn into_response(self) -> Response {
        into_response(T::render(&self.0))
    }
}

#[track_caller]
pub fn into_response(result: askama::Result<String>) -> Response {
    match result {
        Ok(body) => (HEADERS, body).into_response(),
        Err(err) => {
            crate::render_error(&err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

const HEADERS: [(http_1::HeaderName, HeaderValue); 1] = [(
    CONTENT_TYPE,
    HeaderValue::from_static("text/html; charset=utf-8"),
)];
