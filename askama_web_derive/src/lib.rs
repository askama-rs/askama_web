// SPDX-FileCopyrightText: 2025 The Askama Developers
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Please see [`askama_web`](https://crates.io/crates/askama_web) for more information.

use proc_macro2::{Delimiter, Group, TokenTree};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    DeriveInput, ExprPath, GenericParam, Token, WhereClause, bracketed, parenthesized,
    parse_macro_input,
};

/// Implement the needed traits to use your template as a web response.
#[proc_macro_derive(WebTemplate)]
pub fn derive_into_response(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

    let span = Span::call_site();
    let tt = TokenTree::Group(Group::new(Delimiter::Bracket, input));
    let ts = TokenStream::from_iter([tt]);
    TokenStream::from_iter([
        TokenTree::Ident(Ident::new("askama_web", span)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("__askama_web_impl", span)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("derive", span)),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(Delimiter::Brace, ts)),
    ])
}

/// Callback for a framework implementation.
#[proc_macro]
pub fn impl_framework(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ImplFrameWork {
        path,
        bang,
        ast,
        params,
    } = parse_macro_input!(input);
    let ident = &ast.ident;

    let mut generics = ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let mut args = quote! {
        ident: [#ident],
        impl_generics: [#impl_generics],
        ty_generics: [#ty_generics],
        where_clause: [#where_clause],
    };

    if params.generics.is_some() || params.where_clause.is_some() {
        if let Some(params) = params.generics {
            args.extend(quote! {
                ex_params: [#params],
            });
            generics.params.extend(params);
        }

        match (&mut generics.where_clause, params.where_clause) {
            (_, None) => {}
            (dest @ None, src @ Some(_)) => *dest = src,
            (Some(dest), Some(src)) => {
                dest.predicates.extend(src.predicates);
            }
        }

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        args.extend(quote! {
            ex_impl_generics: [#impl_generics],
            ex_ty_generics: [#ty_generics],
            ex_where_clause: [#where_clause],
        });
    }

    let args = TokenTree::Group(Group::new(Delimiter::Brace, args));
    quote!(#path #bang #args).into()
}

struct ImplFrameWork {
    path: ExprPath,
    bang: Token![!],
    ast: DeriveInput,
    params: Params,
}

impl Parse for ImplFrameWork {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let path = input.parse()?;
        let bang = input.parse()?;

        let (ast, params);
        {
            let inner;
            parenthesized!(inner in input);
            Bracketed(ast) = inner.parse()?;
            params = inner.parse()?;
            let _: Eof = inner.parse()?;
        }

        let _: Token![;] = input.parse()?;
        let _: Eof = input.parse()?;
        Ok(Self {
            path,
            bang,
            ast,
            params,
        })
    }
}

struct Bracketed<T>(T);

impl<T: Parse> Parse for Bracketed<T> {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let ast_content;
        bracketed!(ast_content in input);
        let inner: T = ast_content.parse()?;
        let _: Eof = ast_content.parse()?;
        Ok(Self(inner))
    }
}

#[derive(Default)]
struct Params {
    generics: Option<Punctuated<GenericParam, Token![,]>>,
    where_clause: Option<WhereClause>,
}

impl Parse for Params {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let generics = if input.peek(Token![<]) {
            let _: Token![<] = input.parse()?;

            let inner = if !input.peek(Token![>]) {
                let inner = <Punctuated<GenericParam, Token![,]>>::parse_separated_nonempty(input)?;
                if input.peek(Token![,]) {
                    let _: Token![,] = input.parse()?;
                }
                inner
            } else {
                Default::default()
            };

            let _: Token![>] = input.parse()?;
            Some(inner)
        } else {
            None
        };

        let where_clause = if input.peek(Token![where]) {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Self {
            generics,
            where_clause,
        })
    }
}

struct Eof;

impl Parse for Eof {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        if input.is_empty() {
            Ok(Eof)
        } else {
            Err(syn::Error::new(input.span(), "expected end of input"))
        }
    }
}
