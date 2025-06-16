use std::borrow::Cow;
use std::io::Cursor;

pub use askama::Template;
pub use rocket_0_5::Request;
use rocket_0_5::Response;
use rocket_0_5::http::{ContentType, MediaType, Status};
pub use rocket_0_5::response::{Responder, Result};

#[cfg(feature = "derive")]
pub use crate::__askama_web_impl_rocket_0_5 as derive;

#[cfg(feature = "derive")]
#[macro_export]
#[doc(hidden)]
macro_rules! __askama_web_impl_rocket_0_5 {
    (@ $ast:tt) => {
        $crate::__askama_web_impl::askama_web_derive::impl_framework! {
            $crate::__askama_web_impl::rocket_0_5::derive!($ast <'__askama_web_rocket>);
        }
    };
    (
        ident: [$ident:ident],
        impl_generics: $_impl_generics:tt,
        ty_generics: [$($ty_generics:tt)*],
        where_clause: [$($where_clause:tt)*],
        ex_params: [$req:lifetime],
        ex_impl_generics: [$($impl_generics:tt)*],
        ex_ty_generics: $_ty_generics:tt,
        ex_where_clause: $_where_clause:tt,
    ) => {
        const _: () = {
            use $crate::__askama_web_impl::rocket_0_5 as __askama_web;

            impl $($impl_generics)* __askama_web::Responder<$req, 'static>
            for $ident $($ty_generics)* $($where_clause)* {
                #[inline]
                #[track_caller]
                fn respond_to(
                    self,
                    _: &$req __askama_web::Request<'_>,
                ) -> __askama_web::Result<'static> {
                    let result = <Self as __askama_web::Template>::render(&self);
                    __askama_web::Result::Ok(__askama_web::respond_to(result))
                }
            }
        };
    };
}

impl<'r, T: Template> Responder<'r, 'static> for crate::WebTemplate<T> {
    #[inline]
    #[track_caller]
    fn respond_to(self, _: &'r Request<'_>) -> Result<'static> {
        Ok(respond_to(T::render(&self.0)))
    }
}

#[track_caller]
pub fn respond_to(result: askama::Result<String>) -> Response<'static> {
    let (status, content_type, body) = match result {
        Ok(body) => (Status::Ok, HTML, Cow::Owned(body.into_bytes())),
        Err(err) => {
            crate::render_error(&err);
            (Status::InternalServerError, TEXT, FAIL)
        }
    };

    let mut resp = Response::new();
    resp.set_status(status);
    resp.set_header(content_type);
    resp.set_sized_body(body.len(), Cursor::new(body));
    resp
}

const HTML: ContentType = ContentType(MediaType::HTML);
const TEXT: ContentType = ContentType(MediaType::Text);
const FAIL: Cow<'_, [u8]> = Cow::Borrowed(super::FAIL.as_bytes());
