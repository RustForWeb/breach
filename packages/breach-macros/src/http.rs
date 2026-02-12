mod attribute;
mod data;
mod r#enum;
mod r#struct;
mod r#union;

use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use syn::{DeriveInput, Generics, Ident, Result};

use crate::http::data::HttpErrorData;

pub struct HttpError<'a> {
    ident: &'a Ident,
    generics: &'a Generics,
    data: HttpErrorData<'a>,
}

impl<'a> HttpError<'a> {
    pub fn parse(input: &'a DeriveInput) -> Result<Self> {
        Ok(Self {
            ident: &input.ident,
            generics: &input.generics,
            data: HttpErrorData::parse(input)?,
        })
    }
}

impl<'a> ToTokens for HttpError<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

        let status = self.data.status();

        tokens.append_all(quote! {
            #[automatically_derived]
            impl #impl_generics ::breach::HttpError for #ident #type_generics #where_clause {
                fn status(&self) -> ::breach::http::StatusCode {
                    #status
                }
            }
        });

        if let Some(attribute) = self.data.attribute()
            && attribute.axum
        {
            tokens.append_all(quote! {
                #[automatically_derived]
                impl #impl_generics ::axum::response::IntoResponse for #ident #type_generics #where_clause {
                    fn into_response(self) -> ::axum::response::Response {
                        (self.status(), Json(self)).into_response()
                    }
                }
            });
        }
    }
}
