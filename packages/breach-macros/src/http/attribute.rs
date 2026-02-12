use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Error, Ident, Result, spanned::Spanned};

pub struct HttpErrorAttribute {
    status: Option<Ident>,
    pub axum: bool,
}

impl<'a> HttpErrorAttribute {
    pub fn parse_slice(input: &'a [Attribute]) -> Result<Option<Self>> {
        let mut result = None;

        for attribute in input {
            if !attribute.meta.path().is_ident("http") {
                continue;
            }

            if result.is_some() {
                return Err(Error::new(
                    attribute.span(),
                    "only a single `http` attribute is allowed",
                ));
            }

            result = Some(Self::parse(attribute)?);
        }

        Ok(result)
    }

    pub fn parse(attribute: &'a Attribute) -> Result<Self> {
        let mut status = None;
        let mut axum = false;

        attribute.parse_nested_meta(|meta| {
            if meta.path.is_ident("status") {
                status = Some(meta.value()?.parse()?);

                Ok(())
            } else if meta.path.is_ident("axum") {
                axum = true;

                Ok(())
            } else {
                Err(meta.error("unknown parameter"))
            }
        })?;

        Ok(Self { status, axum })
    }

    pub fn status(&self) -> TokenStream {
        if let Some(status) = &self.status {
            if status == "UNPROCESSABLE_CONTENT" {
                quote!(::breach::http::StatusCode::UNPROCESSABLE_ENTITY)
            } else {
                quote!(::breach::http::StatusCode::#status)
            }
        } else {
            quote!(compile_error!("missing `#[http(status = ..)]` attribute"))
        }
    }
}
