// SPDX-FileCopyrightText: 2025 The Askama Developers
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! [![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Kijewski/askama_web/ci.yml?branch=main&style=flat-square&logo=github&logoColor=white "GitHub Workflow Status")](https://github.com/Kijewski/askama_web/actions/workflows/ci.yml)
//! [![Crates.io](https://img.shields.io/crates/v/askama_web?logo=rust&style=flat-square "Crates.io")](https://crates.io/crates/askama_web)
//! [![docs.rs](https://img.shields.io/docsrs/askama_web?logo=docsdotrs&style=flat-square&logoColor=white "docs.rs")](https://docs.rs/askama_web/)
//!
//! A compatibility add-on for [askama](https://lib.rs/crates/askama) to support
//! many different web frameworks.
//!
//! ## Example
//!
//! E.g. if you are using [axum](https://lib.rs/crates/axum), then add `askama_web` with
//! the feature `"axum-0.8"` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! askama_web = { version = "0.0.1-pre.0", features = ["axum-0.8"] }
//! ```
//!
//! Then just add <code>#[derive([WebTemplate])]</code> to your
//! [Askama templated][askama::Template] `struct` or `enum`:
//!
//! ```rust
//! use askama::Template;
//! use askama_web::WebTemplate;
//! # /*
//! use axum::Router;
//! use axum::routing::get;
//! # */
//!
//! #[derive(Template, WebTemplate)]
//! #[template(path = "hello.html")]
//! struct HelloTemplate {
//!     name: String,
//! }
//!
//! async fn hello() -> HelloTemplate {
//!     HelloTemplate {
//!         name: "world".to_string(),
//!     }
//! }
//!
//! # /*
//! let app = Router::new().route("/", get(hello));
//! # */
//! ```
//!
//! By selecting the feature `"axum-0.8"`, `HelloTemplate` will implement [`axum::response::IntoResponse`].
//! The user will receive a "Status: 200", "Content-Type: text/html; charset=utf-8"
//! response with the rendered struct as body.
//!
//! [`axum::response::IntoResponse`]: https://docs.rs/actix-web/4.9.0/actix_web/dev/struct.ServiceResponse.html#method.into_response
//!
//! ## Feature flags / web framework selection
//!
//! These web frameworks are currently implemented
//! and can be selected with their respective feature flag:
//!
//! * `"actix-web-4"`: [actix-web](https://docs.rs/actix-web/4.x.x/) in version `4.x.y`
//! * `"axum-0.8"`: [axum](https://docs.rs/axum/0.8.x/) in version `0.8.y`
//! * `"poem-3"`: [poem](https://docs.rs/poem/3.x.x/) in version `3.x.y`
//! * `"rocket-0.5"`: [rocket](https://docs.rs/rocket/0.5.x/) in version `0.5.y`
//! * `"warp-0.3"`: [warp](https://docs.rs/warp/0.3.x/) in version `0.3.y`
//!
//! As well as these logging / debugging facilities to print error message
//! if a template could not be rendered:
//!
//! * `"eprintln"`: using rust's built-in [`eprintln!()`] macro
//! * `"log-0.4"`: using [log](https://docs.rs/log/0.4.x/) as logging framework
//! * `"tracing-0.1"`: using [tracing](https://docs.rs/tracing/0.1.x/) as logging framework

#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

mod implementation;

#[doc(hidden)]
pub mod __askama_web_impl {
    pub use crate::implementation::*;
}

/// Implement the needed traits to use your template as a web response.
///
/// Please see the [crate] root for more information.
pub use askama_web_derive::WebTemplate;

#[inline]
#[track_caller]
#[allow(unused)]
fn render_error(_err: &askama::Error) {
    #[cfg(feature = "eprintln")]
    eprintln!("Failed to render template: {_err}");
    #[cfg(feature = "log-0.4")]
    log_0_4::error!("Failed to render template: {_err}");
    #[cfg(feature = "tracing-0.1")]
    tracing_0_1::error!("Failed to render template: {_err}");
}
