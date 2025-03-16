use actix_web_4::HttpResponseBuilder;
pub use actix_web_4::body::BoxBody;
use actix_web_4::http::StatusCode;
use actix_web_4::http::header::HeaderValue;
pub use actix_web_4::{HttpRequest, HttpResponse, Responder};
pub use askama::Template;

pub use crate::__askama_web_impl_actix_web_4 as derive;

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
                    req: &__askama_web::HttpRequest,
                ) -> __askama_web::HttpResponse<Self::Body> {
                    let result = <Self as __askama_web::Template>::render(&self);
                    __askama_web::respond_to(result, req)
                }
            }
        };
    };
}

#[track_caller]
pub fn respond_to(result: askama::Result<String>, req: &HttpRequest) -> HttpResponse {
    match result {
        Ok(body) => HttpResponseBuilder::new(StatusCode::OK)
            .content_type(HeaderValue::from_static("text/html; charset=utf-8"))
            .body(body)
            .respond_to(req),
        Err(err) => {
            crate::render_error(&err);
            HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
                .content_type(HeaderValue::from_static("text/plain; charset=utf-8"))
                .body("INTERNAL SERVER ERROR")
                .respond_to(req)
        }
    }
}
