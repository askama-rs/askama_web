pub use askama::Template;
use http_1::StatusCode;
use salvo_core_0_78::writing::Text;
pub use salvo_core_0_78::{Response, Scribe};

#[cfg(feature = "derive")]
pub use crate::__askama_web_impl_salvo_core_0_78 as derive;

#[cfg(feature = "derive")]
#[macro_export]
#[doc(hidden)]
macro_rules! __askama_web_impl_salvo_core_0_78 {
    (@ $ast:tt) => {
        $crate::__askama_web_impl::askama_web_derive::impl_framework! {
            $crate::__askama_web_impl::salvo_core_0_78::derive!($ast);
        }
    };
    (
        ident: [$ident:ident],
        impl_generics: [$($impl_generics:tt)*],
        ty_generics: [$($ty_generics:tt)*],
        where_clause: [$($where_clause:tt)*],
    ) => {
        const _: () = {
            use $crate::__askama_web_impl::salvo_core_0_78 as __askama_web;

            impl $($impl_generics)* __askama_web::Scribe
            for $ident $($ty_generics)* $($where_clause)* {
                #[inline]
                #[track_caller]
                fn render(self, res: &mut __askama_web::Response) {
                    let result = <Self as __askama_web::Template>::render(&self);
                    __askama_web::render(result, res)
                }
            }
        };
    };
}

impl<T: Template> Scribe for crate::WebTemplate<T> {
    #[inline]
    #[track_caller]
    fn render(self, res: &mut Response) {
        render(T::render(&self.0), res)
    }
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
