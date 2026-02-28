use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{DataEnum, DeriveInput, Error, Field, Fields, Ident, Result, Variant, spanned::Spanned};

use crate::http::attribute::{HttpErrorAttribute, HttpErrorDataAttribute};

pub struct HttpErrorEnum<'a> {
    ident: &'a Ident,
    variants: Vec<HttpErrorEnumVariant<'a>>,
    attribute: Option<HttpErrorDataAttribute>,
}

impl<'a> HttpErrorEnum<'a> {
    pub fn parse(input: &'a DeriveInput, data: &'a DataEnum) -> Result<Self> {
        let mut result = HttpErrorEnum {
            ident: &input.ident,
            variants: Vec::with_capacity(data.variants.len()),
            attribute: HttpErrorDataAttribute::parse_slice(&input.attrs)?,
        };

        for variant in &data.variants {
            result
                .variants
                .push(HttpErrorEnumVariant::parse(result.ident, variant)?);
        }

        Ok(result)
    }

    pub fn attribute(&self) -> Option<&HttpErrorDataAttribute> {
        self.attribute.as_ref()
    }

    pub fn status(&self) -> TokenStream {
        let arms = self.variants.iter().map(|variant| variant.status());

        quote! {
            match &self {
                #( #arms ),*
            }
        }
    }

    pub fn responses(&self) -> TokenStream {
        let base = self
            .attribute
            .as_ref()
            .and_then(|attribute| attribute.base.as_ref())
            .map(|r#type| quote!(<#r#type as ::utoipa::IntoResponses>::responses()));

        let mut responses = base
            .into_iter()
            .chain(self.variants.iter().map(|variant| variant.responses()))
            .collect::<Vec<_>>();

        if responses.is_empty() {
            quote!(::std::collections::BTreeMap::default())
        } else if responses.len() == 1 {
            responses.remove(0)
        } else {
            quote! {
                ::breach::utoipa::merge_responses([
                    #( #responses ),*
                ].into_iter())
            }
        }
    }
}

pub struct HttpErrorEnumVariant<'a> {
    enum_ident: &'a Ident,
    ident: &'a Ident,
    fields: &'a Fields,
    field: Option<&'a Field>,
    attribute: Option<HttpErrorAttribute>,
}

impl<'a> HttpErrorEnumVariant<'a> {
    pub fn parse(enum_ident: &'a Ident, variant: &'a Variant) -> Result<Self> {
        let field = match &variant.fields {
            Fields::Named(fields) => {
                return Err(Error::new(fields.span(), "named fields are not supported"));
            }
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() > 1 {
                    return Err(Error::new(
                        fields.unnamed.span(),
                        "multiple unnamed fields are not supported",
                    ));
                }

                fields.unnamed.first().and_then(|field| {
                    if field.attrs.iter().any(|attribute| {
                        if attribute.meta.path().is_ident("serde") {
                            let mut skip = false;

                            _ = attribute.parse_nested_meta(|meta| {
                                if meta.path.is_ident("skip") {
                                    skip = true;
                                }

                                Ok(())
                            });

                            skip
                        } else {
                            false
                        }
                    }) {
                        None
                    } else {
                        Some(field)
                    }
                })
            }
            Fields::Unit => None,
        };

        Ok(HttpErrorEnumVariant {
            enum_ident,
            ident: &variant.ident,
            fields: &variant.fields,
            field,
            attribute: HttpErrorAttribute::parse_slice(&variant.attrs)?,
        })
    }

    pub fn status(&self) -> TokenStream {
        self.arm(if let Some(attribute) = &self.attribute {
            attribute.status()
        } else if self.field.is_some() {
            quote!(value.status())
        } else {
            quote!(compile_error!("missing `#[http(status = ..)]` attribute"))
        })
    }

    pub fn responses(&self) -> TokenStream {
        if let Some(attribute) = &self.attribute {
            attribute.responses(self.field.as_ref().map(|field| field.ty.to_token_stream()))
        } else if let Some(field) = &self.field {
            let r#type = &field.ty;

            quote!(<#r#type as ::utoipa::IntoResponses>::responses())
        } else {
            quote!(compile_error!("missing `#[http(status = ..)]` attribute"))
        }
    }

    fn arm(&self, tokens: TokenStream) -> TokenStream {
        let enum_ident = self.enum_ident;
        let ident = self.ident;

        match self.fields {
            Fields::Named(_) => {
                quote! {
                    #enum_ident::#ident { .. } => #tokens
                }
            }
            Fields::Unnamed(fields) => {
                let idents: Vec<TokenStream> = if self.attribute.is_some() {
                    fields.unnamed.iter().map(|_| quote!(_)).collect()
                } else {
                    fields.unnamed.iter().map(|_| quote!(value)).collect()
                };

                quote! {
                    #enum_ident::#ident( #(#idents),* ) => #tokens
                }
            }
            Fields::Unit => {
                quote! {
                    #enum_ident::#ident => #tokens
                }
            }
        }
    }
}
