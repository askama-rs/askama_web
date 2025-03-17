pub use std::borrow::Cow;
pub use std::boxed::Box;
pub use std::future::Future;
pub use std::marker::Send;
pub use std::pin::Pin;
pub use std::primitive::str;
pub use std::stringify;

pub use askama::Template;
use trillium_0_2::{Body, KnownHeaderName, Status};
pub use trillium_0_2::{Conn, Handler};

pub use crate::__askama_web_impl_trillium_0_2 as derive;

#[macro_export]
#[doc(hidden)]
macro_rules! __askama_web_impl_trillium_0_2 {
    (@ $ast:tt) => {
        $crate::__askama_web_impl::askama_web_derive::impl_framework! {
            $crate::__askama_web_impl::trillium_0_2::derive!(
                $ast where Self: Send + Sync + 'static
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
    let (status, ct, body) = if let Some(body) = result {
        (
            Status::Ok,
            "text/html; charset=utf-8",
            Body::new_static(body.into_bytes()),
        )
    } else {
        (
            Status::InternalServerError,
            "text/plain; charset=utf-8",
            Body::new_static(b"INTERNAL SERVER ERROR"),
        )
    };
    conn.with_status(status)
        .with_response_header(KnownHeaderName::ContentType, ct)
        .with_body(body)
}
