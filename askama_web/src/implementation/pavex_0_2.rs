pub use askama::Template;
pub use pavex_0_2::IntoResponse;
pub use pavex_0_2::Response;

#[cfg(feature = "derive")]
pub use crate::__askama_web_impl_pavex_0_2 as derive;

#[cfg(feature = "derive")]
#[macro_export]
#[doc(hidden)]
macro_rules! __askama_web_impl_pavex_0_2 {
    (@ $ast:tt) => {
        $crate::__askama_web_impl::askama_web_derive::impl_framework! {
            $crate::__askama_web_impl::pavex_0_2::derive!($ast);
        }
    };
    (
        ident: [$ident:ident],
        impl_generics: [$($impl_generics:tt)*],
        ty_generics: [$($ty_generics:tt)*],
        where_clause: [$($where_clause:tt)*],
    ) => {
        const _: () = {
            use $crate::__askama_web_impl::pavex_0_2 as __askama_web;

            impl $($impl_generics)* __askama_web::IntoResponse
            for $ident $($ty_generics)* $($where_clause)* {
                #[inline]
                #[track_caller]
                fn into_response(self) -> $crate::__askama_web_impl::pavex_0_2::Response {
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
    match result {
        Ok(body) => {
            let html = pavex_0_2::response::body::Html::from(body);
            Response::ok().set_typed_body(html)
        }
        Err(err) => {
            crate::render_error(&err);
            Response::internal_server_error()
        }
    }
}
