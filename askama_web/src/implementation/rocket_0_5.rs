pub use askama::Template;
pub use rocket_0_5::Request;
use rocket_0_5::http::Status;
use rocket_0_5::response::content::RawHtml;
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
                    request: &$req __askama_web::Request<'_>,
                ) -> __askama_web::Result<'static> {
                    let result = <Self as __askama_web::Template>::render(&self);
                    __askama_web::respond_to(result, request)
                }
            }
        };
    };
}

impl<'r, T: Template> Responder<'r, 'static> for crate::WebTemplate<T> {
    #[inline]
    #[track_caller]
    fn respond_to(self, request: &'r Request<'_>) -> Result<'static> {
        respond_to(T::render(&self.0), request)
    }
}

#[track_caller]
pub fn respond_to(result: askama::Result<String>, request: &Request<'_>) -> Result<'static> {
    match result {
        Ok(body) => RawHtml(body).respond_to(request),
        Err(err) => {
            crate::render_error(&err);
            Status::InternalServerError.respond_to(request)
        }
    }
}
