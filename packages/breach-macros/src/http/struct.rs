use proc_macro2::TokenStream;
use syn::{DataStruct, DeriveInput, Error, Result, spanned::Spanned};

use crate::http::attribute::HttpErrorAttribute;

pub struct HttpErrorStruct {
    attribute: HttpErrorAttribute,
}

impl<'a> HttpErrorStruct {
    pub fn parse(input: &'a DeriveInput, _data: &'a DataStruct) -> Result<Self> {
        let Some(attribute) = HttpErrorAttribute::parse_slice(&input.attrs)? else {
            return Err(Error::new(input.span(), "missing http attribute"));
        };

        Ok(HttpErrorStruct { attribute })
    }

    pub fn attribute(&self) -> Option<&HttpErrorAttribute> {
        Some(&self.attribute)
    }

    pub fn status(&self) -> TokenStream {
        self.attribute.status()
    }
}
