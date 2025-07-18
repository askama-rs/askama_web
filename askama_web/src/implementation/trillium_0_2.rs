pub use std::borrow::Cow;
pub use std::boxed::Box;
pub use std::future::Future;
pub use std::marker::{Send, Sync};
pub use std::pin::Pin;
pub use std::primitive::str;
pub use std::stringify;

pub use askama::Template;
pub use trillium_0_2::{Conn, Handler};
use trillium_0_2::{KnownHeaderName, Status};

#[cfg(feature = "derive")]
pub use crate::__askama_web_impl_trillium_0_2 as derive;

#[cfg(feature = "derive")]
#[macro_export]
#[doc(hidden)]
macro_rules! __askama_web_impl_trillium_0_2 {
    (@ $ast:tt) => {
        $crate::__askama_web_impl::askama_web_derive::impl_framework! {
            $crate::__askama_web_impl::trillium_0_2::derive!(
                $ast
                where
                    Self: $crate::__askama_web_impl::trillium_0_2::Send
                        + $crate::__askama_web_impl::trillium_0_2::Sync
                        + 'static
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
            use $crate::__askama_web_impl::trillium_0_2 as __askama_web;

            impl $($impl_generics)* __askama_web::Handler
            for $ident $($ty_generics)* $($where_clause)* {
                #[inline]
                #[must_use]
                #[track_caller]
                #[allow(single_use_lifetimes)] // false-positive
                fn run<'__askama_web_self, '__askama_web_future>(
                    &'__askama_web_self self,
                    conn: __askama_web::Conn,
                ) -> __askama_web::Pin<Box<
                    dyn __askama_web::Future<Output = __askama_web::Conn> +
                    __askama_web::Send +
                    '__askama_web_future
                >>
                where
                    '__askama_web_self: '__askama_web_future,
                    Self: '__askama_web_future,
                {
                    let result = <Self as __askama_web::Template>::render(&self);
                    __askama_web::render(result, conn)
                }

                #[inline]
                fn name(&self) -> __askama_web::Cow<'static, __askama_web::str> {
                    __askama_web::Cow::Borrowed(__askama_web::stringify!($ident))
                }
            }
        };
    };
}

impl<T: Template + Send + Sync + 'static> Handler for crate::WebTemplate<T> {
    #[inline]
    #[track_caller]
    #[allow(single_use_lifetimes)] // false-positive
    fn run<'a: 'b, 'b>(&'a self, conn: Conn) -> Pin<Box<dyn Future<Output = Conn> + Send + 'b>>
    where
        Self: 'b,
    {
        render(T::render(&self.0), conn)
    }
}

#[must_use]
#[track_caller]
pub fn render(
    result: askama::Result<String>,
    conn: Conn,
) -> Pin<Box<dyn Future<Output = Conn> + Send + 'static>> {
    let result = match result {
        Ok(body) => Some(body),
        Err(err) => {
            crate::render_error(&err);
            None
        }
    };
    Box::pin(run(result, conn))
}

#[must_use]
async fn run(result: Option<String>, conn: Conn) -> Conn {
    let (status, content_type, body) = match result {
        Some(body) => (Status::Ok, HTML, Cow::Owned(body.into_bytes())),
        None => (Status::InternalServerError, TEXT, FAIL),
    };

    conn.with_status(status)
        .with_response_header(KnownHeaderName::ContentType, content_type)
        .with_body(body)
}

const HTML: &str = super::HTML;
const TEXT: &str = super::TEXT;
const FAIL: Cow<'_, [u8]> = Cow::Borrowed(super::FAIL.as_bytes());
