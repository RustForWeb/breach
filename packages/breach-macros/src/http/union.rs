use proc_macro2::TokenStream;
use syn::{DataUnion, DeriveInput, Result};

use crate::http::attribute::HttpErrorDataAttribute;

pub struct HttpErrorUnion {}

impl HttpErrorUnion {
    pub fn parse(input: &DeriveInput, _data: &DataUnion) -> Result<Self> {
        Err(syn::Error::new_spanned(input, "union is not supported"))
    }

    pub fn attribute(&self) -> Option<&HttpErrorDataAttribute> {
        None
    }

    pub fn status(&self) -> TokenStream {
        todo!()
    }

    pub fn responses(&self) -> TokenStream {
        todo!()
    }

    pub fn hook(&self) -> TokenStream {
        todo!()
    }
}
