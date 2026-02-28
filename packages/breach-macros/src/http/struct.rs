use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, DeriveInput, Error, Result, spanned::Spanned};

use crate::http::attribute::HttpErrorDataAttribute;

pub struct HttpErrorStruct {
    attribute: HttpErrorDataAttribute,
}

impl<'a> HttpErrorStruct {
    pub fn parse(input: &'a DeriveInput, _data: &'a DataStruct) -> Result<Self> {
        let Some(attribute) = HttpErrorDataAttribute::parse_slice(&input.attrs)? else {
            return Err(Error::new(input.span(), "missing http attribute"));
        };

        Ok(HttpErrorStruct { attribute })
    }

    pub fn attribute(&self) -> Option<&HttpErrorDataAttribute> {
        Some(&self.attribute)
    }

    pub fn status(&self) -> TokenStream {
        self.attribute.status()
    }

    pub fn responses(&self) -> TokenStream {
        self.attribute.responses(Some(quote!(Self)))
    }
}
