pub use std::marker::Send;

pub use askama::Template;
use bytes_1::Bytes;
use http_1::header::{CONTENT_TYPE, HeaderValue};
use http_1::{Extensions, HeaderMap, StatusCode, Version};
use poem_3::{Body, ResponseParts};
pub use poem_3::{IntoResponse, Response};

#[cfg(feature = "derive")]
pub use crate::__askama_web_impl_poem_3 as derive;

#[cfg(feature = "derive")]
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

impl<T: Template + Send> IntoResponse for crate::WebTemplate<T> {
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

    let mut parts = ResponseParts {
        status,
        version: Version::default(),
        headers: HeaderMap::with_capacity(1),
        extensions: Extensions::default(),
    };
    parts.headers.insert(CONTENT_TYPE, content_type);
    Response::from_parts(parts, Body::from_bytes(body))
}

const HTML: HeaderValue = HeaderValue::from_static(super::HTML);
const TEXT: HeaderValue = HeaderValue::from_static(super::TEXT);
const FAIL: Bytes = Bytes::from_static(super::FAIL.as_bytes());
