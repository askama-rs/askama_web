pub use askama::Template;
use axum_core_0_4::body::Body;
pub use axum_core_0_4::response::{IntoResponse, Response};
use bytes_1::Bytes;
use http_1::StatusCode;
use http_1::header::{CONTENT_TYPE, HeaderValue};

#[cfg(feature = "derive")]
pub use crate::__askama_web_impl_axum_core_0_4 as derive;

#[cfg(feature = "derive")]
#[macro_export]
#[doc(hidden)]
macro_rules! __askama_web_impl_axum_core_0_4 {
    (@ $ast:tt) => {
        $crate::__askama_web_impl::askama_web_derive::impl_framework! {
            $crate::__askama_web_impl::axum_core_0_4::derive!($ast);
        }
    };
    (
        ident: [$ident:ident],
        impl_generics: [$($impl_generics:tt)*],
        ty_generics: [$($ty_generics:tt)*],
        where_clause: [$($where_clause:tt)*],
    ) => {
        const _: () = {
            use $crate::__askama_web_impl::axum_core_0_4 as __askama_web;

            impl $($impl_generics)* __askama_web::IntoResponse
            for $ident $($ty_generics)* $($where_clause)* {
                #[inline]
                #[track_caller]
                fn into_response(self) -> $crate::__askama_web_impl::axum_core_0_4::Response {
                    let result = <Self as __askama_web::Template>::render(&self);
                    __askama_web::into_response(result)
                }
            }
        };
    };
}

impl<T: Template> IntoResponse for crate::WebTemplate<T> {
    #[inline]
    #[track_caller]
    fn into_response(self) -> Response {
        into_response(T::render(&self.0))
    }
}

#[track_caller]
pub fn into_response(result: askama::Result<String>) -> Response {
    let (status, content_type, body) = match result {
        Ok(body) => (StatusCode::OK, HTML, Bytes::from_owner(body)),
        Err(err) => {
            crate::render_error(&err);
            (StatusCode::INTERNAL_SERVER_ERROR, TEXT, FAIL)
        }
    };

    let mut resp = Body::from(body).into_response();
    *resp.status_mut() = status;
    resp.headers_mut().insert(CONTENT_TYPE, content_type);
    resp
}

const HTML: HeaderValue = HeaderValue::from_static(super::HTML);
const TEXT: HeaderValue = HeaderValue::from_static(super::TEXT);
const FAIL: Bytes = Bytes::from_static(super::FAIL.as_bytes());
