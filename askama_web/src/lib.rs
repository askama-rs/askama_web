// SPDX-FileCopyrightText: 2025-2026 The Askama Developers
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[doc(hidden)]
mod implementation;

#[doc(hidden)]
#[cfg(feature = "derive")]
pub mod __askama_web_impl {
    pub use crate::implementation::*;
}

use std::fmt;

use askama::{FastWritable, Template, Values};
#[cfg(feature = "derive")]
/// Implement the needed traits to use your template as a web response.
///
/// Instead of this proc-macro, you can also you manually wrap a [`Template`] in
/// [`WebTemplate`][struct@WebTemplate] or use [`WebTemplateExt::into_web_template()`].
///
/// Please see the [crate] root for more information.
pub use askama_web_derive::WebTemplate;

/// Extension trait to let any [`Template`] be usable as a [`WebTemplate`][derive@WebTemplate].
pub trait WebTemplateExt: Template {
    /// Treat a reference to a [`Template`] as [`WebTemplate`][struct@WebTemplate].
    ///
    /// In most cases [`.into_web_template()`][WebTemplateExt::into_web_template] will work better.
    fn as_web_template(&self) -> WebTemplate<&Self>;

    /// Treat a [`Template`] as [`WebTemplate`][struct@WebTemplate].
    fn into_web_template(self) -> WebTemplate<Self>
    where
        Self: Sized;
}

impl<T: Template> WebTemplateExt for T {
    #[inline]
    fn as_web_template(&self) -> WebTemplate<&Self> {
        WebTemplate(self)
    }

    #[inline]
    fn into_web_template(self) -> WebTemplate<Self> {
        WebTemplate(self)
    }
}

/// Wrap a [`Template`] that might not derive [`WebTemplate`][derive@WebTemplate] to be usable as
/// web response.
///
/// A [`Template`] wrapped in this `struct` implements all traits that were enabled through feature
/// flags. Please see the [crate] documentation for a list of all supported feature flags / web
/// frameworks.
///
/// You might also find [`WebTemplateExt::into_web_template()`] convenient.
#[derive(Debug, Clone, Copy)]
pub struct WebTemplate<T: Template>(pub T);

impl<T: Template> Template for WebTemplate<T> {
    #[inline]
    fn render(&self) -> askama::Result<String> {
        <T as Template>::render(&self.0)
    }

    #[inline]
    fn render_with_values(&self, values: &dyn Values) -> askama::Result<String> {
        <T as Template>::render_with_values(&self.0, values)
    }

    #[inline]
    fn render_into_with_values(
        &self,
        writer: &mut dyn fmt::Write,
        values: &dyn Values,
    ) -> askama::Result<()> {
        <T as Template>::render_into_with_values(&self.0, writer, values)
    }

    #[inline]
    fn render_into(&self, writer: &mut dyn fmt::Write) -> askama::Result<()> {
        <T as Template>::render_into(&self.0, writer)
    }

    #[inline]
    fn write_into(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        <T as Template>::write_into(&self.0, writer)
    }

    #[inline]
    fn write_into_with_values(
        &self,
        writer: &mut dyn std::io::Write,
        values: &dyn Values,
    ) -> std::io::Result<()> {
        <T as Template>::write_into_with_values(&self.0, writer, values)
    }

    const SIZE_HINT: usize = <T as Template>::SIZE_HINT;
}

impl<T: Template> FastWritable for WebTemplate<T> {
    #[inline]
    fn write_into(&self, dest: &mut dyn fmt::Write, values: &dyn Values) -> askama::Result<()> {
        <T as FastWritable>::write_into(&self.0, dest, values)
    }
}

/// Implement the [`format!()`] trait for `WebTemplate<T>`.
///
/// Please be aware of the rendering performance notice in the [`Template`] trait.
impl<T: Template> fmt::Display for WebTemplate<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <T as fmt::Display>::fmt(&self.0, f)
    }
}

#[inline]
#[track_caller]
#[allow(unused)]
fn render_error(_err: &(impl std::error::Error + ?Sized)) {
    #[cfg(feature = "eprintln")]
    eprintln!(
        "[{}] Failed to render template: {_err}",
        std::panic::Location::caller(),
    );
    #[cfg(feature = "log-0.4")]
    log_0_4::error!("Failed to render template: {_err}");
    #[cfg(feature = "tracing-0.1")]
    tracing_0_1::error!("Failed to render template: {_err}");
}
