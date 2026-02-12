#![warn(missing_docs)]

//! Breach macros.

mod http;
mod util;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{DeriveInput, Error, parse_macro_input};

use crate::http::HttpError;

/// `HttpError` derive macro.
#[proc_macro_derive(HttpError, attributes(http))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    HttpError::parse(&input)
        .map(|http_error| http_error.to_token_stream())
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
