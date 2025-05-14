pub use actix_web_4::body::BoxBody;
use actix_web_4::http::StatusCode;
use actix_web_4::http::header::{CONTENT_TYPE, HeaderValue};
pub use actix_web_4::{HttpRequest, HttpResponse, Responder};
pub use askama::Template;
use bytes_1::Bytes;

#[cfg(feature = "derive")]
pub use crate::__askama_web_impl_actix_web_4 as derive;

#[cfg(feature = "derive")]
#[macro_export]
#[doc(hidden)]
macro_rules! __askama_web_impl_actix_web_4 {
    (@ $ast:tt) => {
        $crate::__askama_web_impl::askama_web_derive::impl_framework! {
            $crate::__askama_web_impl::actix_web_4::derive!($ast);
        }
    };
    (
        ident: [$ident:ident],
        impl_generics: [$($impl_generics:tt)*],
        ty_generics: [$($ty_generics:tt)*],
        where_clause: [$($where_clause:tt)*],
    ) => {
        const _: () = {
            use $crate::__askama_web_impl::actix_web_4 as __askama_web;

            impl $($impl_generics)* __askama_web::Responder
            for $ident $($ty_generics)* $($where_clause)* {
                type Body = __askama_web::BoxBody;

                #[inline]
                #[track_caller]
                fn respond_to(
                    self,
                    _: &__askama_web::HttpRequest,
                ) -> __askama_web::HttpResponse<Self::Body> {
                    let result = <Self as __askama_web::Template>::render(&self);
                    __askama_web::respond_to(result)
                }
            }
        };
    };
}

impl<T: Template> Responder for crate::WebTemplate<T> {
    type Body = BoxBody;

    #[inline]
    #[track_caller]
    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        respond_to(T::render(&self.0))
    }
}

#[track_caller]
pub fn respond_to(result: askama::Result<String>) -> HttpResponse {
    let (status, content_type, body) = match result {
        Ok(body) => (StatusCode::OK, HTML, Bytes::from_owner(body)),
        Err(err) => {
            crate::render_error(&err);
            (StatusCode::INTERNAL_SERVER_ERROR, TEXT, FAIL)
        }
    };

    let mut resp = HttpResponse::new(status);
    resp.headers_mut().insert(CONTENT_TYPE, content_type);
    resp.set_body(BoxBody::new(body))
}

const HTML: HeaderValue = HeaderValue::from_static(super::HTML);
const TEXT: HeaderValue = HeaderValue::from_static(super::TEXT);
const FAIL: Bytes = Bytes::from_static(super::FAIL.as_bytes());
