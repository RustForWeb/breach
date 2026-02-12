use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, Result};

use crate::http::{
    attribute::HttpErrorAttribute, r#enum::HttpErrorEnum, r#struct::HttpErrorStruct,
    union::HttpErrorUnion,
};

pub enum HttpErrorData<'a> {
    Struct(HttpErrorStruct),
    Enum(HttpErrorEnum<'a>),
    Union(HttpErrorUnion),
}

impl<'a> HttpErrorData<'a> {
    pub fn parse(input: &'a DeriveInput) -> Result<Self> {
        Ok(match &input.data {
            Data::Struct(data) => Self::Struct(HttpErrorStruct::parse(input, data)?),
            Data::Enum(data) => Self::Enum(HttpErrorEnum::parse(input, data)?),
            Data::Union(data) => Self::Union(HttpErrorUnion::parse(input, data)?),
        })
    }

    pub fn attribute(&self) -> Option<&HttpErrorAttribute> {
        match self {
            HttpErrorData::Struct(r#struct) => r#struct.attribute(),
            HttpErrorData::Enum(r#enum) => r#enum.attribute(),
            HttpErrorData::Union(r#union) => r#union.attribute(),
        }
    }

    pub fn status(&self) -> TokenStream {
        match self {
            HttpErrorData::Struct(r#struct) => r#struct.status(),
            HttpErrorData::Enum(r#enum) => r#enum.status(),
            HttpErrorData::Union(r#union) => r#union.status(),
        }
    }
}
