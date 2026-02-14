use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Error, Result, spanned::Spanned};

use crate::status::Status;

pub struct HttpErrorAttribute {
    pub status: Option<Status>,
    pub axum: bool,
    pub utoipa: bool,
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
        let mut utoipa = false;

        attribute.parse_nested_meta(|meta| {
            if meta.path.is_ident("status") {
                status = Some(meta.value()?.parse()?);

                Ok(())
            } else if meta.path.is_ident("axum") {
                axum = true;

                Ok(())
            } else if meta.path.is_ident("utoipa") {
                utoipa = true;

                Ok(())
            } else {
                Err(meta.error("unknown parameter"))
            }
        })?;

        Ok(Self {
            status,
            axum,
            utoipa,
        })
    }

    pub fn status(&self) -> TokenStream {
        if let Some(status) = &self.status {
            let status = status.as_ident();

            quote!(::breach::http::StatusCode::#status)
        } else {
            quote!(compile_error!("missing `#[http(status = ..)]` attribute"))
        }
    }

    pub fn responses(&self, r#type: Option<TokenStream>) -> TokenStream {
        if let Some(status) = &self.status {
            let code = status.code.as_str();

            let content = r#type.map(|r#type| {
                // TODO: Attempt to infer content type from schema?
                quote! {
                    .content(
                        "application/json",
                        ::utoipa::openapi::content::ContentBuilder::new()
                            .schema(Some(<#r#type as ::utoipa::PartialSchema>::schema()))
                            .build()
                    )
                }
            });

            quote! {
                ::std::collections::BTreeMap::from_iter([
                    (
                        #code.to_owned(),
                        ::utoipa::openapi::RefOr::T(
                            ::utoipa::openapi::response::ResponseBuilder::new()
                                #content
                                .build()
                        ),
                    ),
                ])
            }
        } else {
            quote!(compile_error!("missing `#[http(status = ..)]` attribute"))
        }
    }
}
