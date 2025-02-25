pub use askama::Template;
use http_1::StatusCode;
use salvo_core_0_76::writing::Text;
pub use salvo_core_0_76::{Response, Scribe};

pub use crate::__askama_web_impl_salvo_core_0_76 as derive;

#[macro_export]
#[doc(hidden)]
macro_rules! __askama_web_impl_salvo_core_0_76 {
    (@ $ast:tt) => {
        $crate::__askama_web_impl::askama_web_derive::impl_framework! {
            $crate::__askama_web_impl::salvo_core_0_76::derive!($ast);
        }
    };
    (
        ident: [$ident:ident],
        impl_generics: [$($impl_generics:tt)*],
        ty_generics: [$($ty_generics:tt)*],
        where_clause: [$($where_clause:tt)*],
    ) => {
        const _: () = {
            use $crate::__askama_web_impl::salvo_core_0_76 as __askama_web;

            impl $($impl_generics)* __askama_web::Scribe
            for $ident $($ty_generics)* $($where_clause)* {
                #[inline]
                #[track_caller]
                fn render(self, res: &mut __askama_web::Response) {
                    __askama_web::render(<_ as __askama_web::Template>::render(&self), res)
                }
            }
        };
    };
}

#[track_caller]
pub fn render(result: askama::Result<String>, res: &mut Response) {
    match result {
        Ok(body) => Text::Html(body).render(res),
        Err(err) => {
            crate::render_error(&err);
            StatusCode::INTERNAL_SERVER_ERROR.render(res)
        }
    }
}
