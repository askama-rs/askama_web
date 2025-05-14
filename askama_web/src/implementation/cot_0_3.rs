pub use askama::Template;
pub use cot_0_3::Result;
use cot_0_3::bytes::Bytes;
use cot_0_3::http::header::{CONTENT_TYPE, HeaderValue};
pub use cot_0_3::response::{IntoResponse, Response};
use cot_0_3::{Body, StatusCode};

#[cfg(feature = "derive")]
pub use crate::__askama_web_impl_cot_0_3 as derive;

#[cfg(feature = "derive")]
#[macro_export]
#[doc(hidden)]
macro_rules! __askama_web_impl_cot_0_3 {
    (@ $ast:tt) => {
        $crate::__askama_web_impl::askama_web_derive::impl_framework! {
            $crate::__askama_web_impl::cot_0_3::derive!($ast);
        }
    };
    (
        ident: [$ident:ident],
        impl_generics: [$($impl_generics:tt)*],
        ty_generics: [$($ty_generics:tt)*],
        where_clause: [$($where_clause:tt)*],
    ) => {
        const _: () = {
            use $crate::__askama_web_impl::cot_0_3 as __askama_web;

            impl $($impl_generics)* __askama_web::IntoResponse
            for $ident $($ty_generics)* $($where_clause)* {
                #[inline]
                #[track_caller]
                fn into_response(self) -> __askama_web::Result<__askama_web::Response> {
                    let result = <Self as __askama_web::Template>::render(&self);
                    __askama_web::Result::Ok(__askama_web::into_response(result))
                }
            }
        };
    };
}

impl<T: Template> IntoResponse for crate::WebTemplate<T> {
    #[inline]
    #[track_caller]
    fn into_response(self) -> Result<Response> {
        Ok(into_response(T::render(&self.0)))
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

    let mut resp = Response::new(Body::fixed(body));
    *resp.status_mut() = status;
    resp.headers_mut().insert(CONTENT_TYPE, content_type);
    resp
}

const HTML: HeaderValue = HeaderValue::from_static(super::HTML);
const TEXT: HeaderValue = HeaderValue::from_static(super::TEXT);
const FAIL: Bytes = Bytes::from_static(super::FAIL.as_bytes());
