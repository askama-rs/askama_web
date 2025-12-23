#[cfg(feature = "derive")]
pub use askama_web_derive;

#[cfg(feature = "derive")]
#[doc(inline)]
pub use crate::__askama_web_impl as derive;

#[cfg(feature = "derive")]
pub mod noop {
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __askama_web_impl_noop {
        (@ $ast:tt) => {};
    }

    pub use crate::__askama_web_impl_noop as derive;
}

#[cfg(feature = "actix-web-4")]
pub mod actix_web_4;
#[cfg(feature = "axum-core-0.4")]
pub mod axum_core_0_4;
#[cfg(feature = "axum-core-0.5")]
pub mod axum_core_0_5;
#[cfg(feature = "cot-0.3")]
pub mod cot_0_3;
#[cfg(feature = "cot-0.4")]
pub mod cot_0_4;
#[cfg(feature = "poem-3")]
pub mod poem_3;
#[cfg(feature = "rocket-0.5")]
pub mod rocket_0_5;
#[cfg(feature = "trillium-0.2")]
pub mod trillium_0_2;
#[cfg(feature = "warp-0.3")]
pub mod warp_0_3;
#[cfg(feature = "warp-0.4")]
pub mod warp_0_4;

#[cfg(all(feature = "derive", not(feature = "actix-web-4")))]
pub use noop as actix_web_4;
#[cfg(all(feature = "derive", not(feature = "axum-core-0.4")))]
pub use noop as axum_core_0_4;
#[cfg(all(feature = "derive", not(feature = "axum-core-0.5")))]
pub use noop as axum_core_0_5;
#[cfg(all(feature = "derive", not(feature = "cot-0.3")))]
pub use noop as cot_0_3;
#[cfg(all(feature = "derive", not(feature = "cot-0.4")))]
pub use noop as cot_0_4;
#[cfg(all(feature = "derive", not(feature = "poem-3")))]
pub use noop as poem_3;
#[cfg(all(feature = "derive", not(feature = "rocket-0.5")))]
pub use noop as rocket_0_5;
#[cfg(all(feature = "derive", not(feature = "trillium-0.2")))]
pub use noop as trillium_0_2;
#[cfg(all(feature = "derive", not(feature = "warp-0.3")))]
pub use noop as warp_0_3;
#[cfg(all(feature = "derive", not(feature = "warp-0.4")))]
pub use noop as warp_0_4;

#[cfg(feature = "derive")]
#[macro_export]
#[doc(hidden)]
macro_rules! __askama_web_impl {
    ($ast:tt) => {
        $crate::__askama_web_impl::actix_web_4::derive!(@ $ast);
        $crate::__askama_web_impl::axum_core_0_4::derive!(@ $ast);
        $crate::__askama_web_impl::axum_core_0_5::derive!(@ $ast);
        $crate::__askama_web_impl::cot_0_3::derive!(@ $ast);
        $crate::__askama_web_impl::cot_0_4::derive!(@ $ast);
        $crate::__askama_web_impl::poem_3::derive!(@ $ast);
        $crate::__askama_web_impl::rocket_0_5::derive!(@ $ast);
        $crate::__askama_web_impl::trillium_0_2::derive!(@ $ast);
        $crate::__askama_web_impl::warp_0_3::derive!(@ $ast);
        $crate::__askama_web_impl::warp_0_4::derive!(@ $ast);
    };
}

#[allow(unused)]
const HTML: &str = "text/html; charset=utf-8";

#[allow(unused)]
const TEXT: &str = "text/plain; charset=utf-8";

#[allow(unused)]
const FAIL: &str = "INTERNAL SERVER ERROR";
