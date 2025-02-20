pub use askama_web_derive;

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
#[cfg(feature = "axum-core-0.5")]
pub mod axum_core_0_5;
#[cfg(feature = "poem-3")]
pub mod poem_3;
#[cfg(feature = "rocket-0.5")]
pub mod rocket_0_5;
#[cfg(feature = "warp-0.3")]
pub mod warp_0_3;

#[cfg(not(feature = "actix-web-4"))]
pub use noop as actix_web_4;
#[cfg(not(feature = "axum-core-0.5"))]
pub use noop as axum_core_0_5;
#[cfg(not(feature = "poem-3"))]
pub use noop as poem_3;
#[cfg(not(feature = "rocket-0.5"))]
pub use noop as rocket_0_5;
#[cfg(not(feature = "warp-0.3"))]
pub use noop as warp_0_3;

#[macro_export]
#[doc(hidden)]
macro_rules! __askama_web_impl {
    ($ast:tt) => {
        $crate::__askama_web_impl::actix_web_4::derive!(@ $ast);
        $crate::__askama_web_impl::axum_core_0_5::derive!(@ $ast);
        $crate::__askama_web_impl::poem_3::derive!(@ $ast);
        $crate::__askama_web_impl::rocket_0_5::derive!(@ $ast);
        $crate::__askama_web_impl::warp_0_3::derive!(@ $ast);
    };
}

#[doc(inline)]
pub use __askama_web_impl as derive;
