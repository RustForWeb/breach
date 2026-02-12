use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, DeriveInput, Error, Field, Fields, Ident, Result, Variant, spanned::Spanned};

use crate::{http::attribute::HttpErrorAttribute, util::Either};

pub struct HttpErrorEnum<'a> {
    ident: &'a Ident,
    variants: Vec<HttpErrorEnumVariant<'a>>,
    attribute: Option<HttpErrorAttribute>,
}

impl<'a> HttpErrorEnum<'a> {
    pub fn parse(input: &'a DeriveInput, data: &'a DataEnum) -> Result<Self> {
        let mut result = HttpErrorEnum {
            ident: &input.ident,
            variants: Vec::with_capacity(data.variants.len()),
            attribute: HttpErrorAttribute::parse_slice(&input.attrs)?,
        };

        for variant in &data.variants {
            result
                .variants
                .push(HttpErrorEnumVariant::parse(result.ident, variant)?);
        }

        Ok(result)
    }

    pub fn attribute(&self) -> Option<&HttpErrorAttribute> {
        self.attribute.as_ref()
    }

    pub fn status(&self) -> TokenStream {
        let arms = self.variants.iter().map(|variant| variant.status());

        quote! {
            match &self {
                #(#arms),*
            }
        }
    }
}

pub struct HttpErrorEnumVariant<'a> {
    enum_ident: &'a Ident,
    ident: &'a Ident,
    fields: &'a Fields,
    attribute_or_field: Either<HttpErrorAttribute, &'a Field>,
}

impl<'a> HttpErrorEnumVariant<'a> {
    pub fn parse(enum_ident: &'a Ident, variant: &'a Variant) -> Result<Self> {
        let attribute_or_field =
            if let Some(attribute) = HttpErrorAttribute::parse_slice(&variant.attrs)? {
                Either::Left(attribute)
            } else {
                Either::Right(match &variant.fields {
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
                        let Some(field) = fields.unnamed.first() else {
                            return Err(Error::new(
                                fields.unnamed.span(),
                                "no unnamed fields are not supported",
                            ));
                        };

                        field
                    }
                    Fields::Unit => {
                        return Err(Error::new(variant.span(), "unit fields are not supported"));
                    }
                })
            };

        Ok(HttpErrorEnumVariant {
            enum_ident,
            ident: &variant.ident,
            fields: &variant.fields,
            attribute_or_field,
        })
    }

    pub fn status(&self) -> TokenStream {
        let enum_ident = self.enum_ident;
        let ident = self.ident;

        let status = match &self.attribute_or_field {
            Either::Left(attribute) => attribute.status(),
            Either::Right(_field) => quote!(value.status()),
        };

        match self.fields {
            Fields::Named(_) => {
                quote! {
                    #enum_ident::#ident { .. } => #status
                }
            }
            Fields::Unnamed(fields) => {
                let idents: Vec<TokenStream> = if self.attribute_or_field.is_right() {
                    fields.unnamed.iter().map(|_| quote!(value)).collect()
                } else {
                    fields.unnamed.iter().map(|_| quote!(_)).collect()
                };

                quote! {
                    #enum_ident::#ident( #(#idents),* ) => #status
                }
            }
            Fields::Unit => {
                quote! {
                    #enum_ident::#ident => #status
                }
            }
        }
    }
}
