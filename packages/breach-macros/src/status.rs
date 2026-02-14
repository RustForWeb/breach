use http::StatusCode;
use proc_macro2::Span;
use syn::{
    Error, Ident, LitInt, Result,
    parse::{Parse, ParseStream},
};

#[derive(Clone)]
pub struct Status {
    pub code: StatusCode,
    raw: RawStatusCode,
}

impl Status {
    fn as_text(&self) -> &'static str {
        match self.code {
            StatusCode::CONTINUE => "CONTINUE",
            StatusCode::SWITCHING_PROTOCOLS => "SWITCHING_PROTOCOLS",
            StatusCode::PROCESSING => "PROCESSING",
            StatusCode::EARLY_HINTS => "EARLY_HINTS",
            StatusCode::OK => "OK",
            StatusCode::CREATED => "CREATED",
            StatusCode::ACCEPTED => "ACCEPTED",
            StatusCode::NON_AUTHORITATIVE_INFORMATION => "NON_AUTHORITATIVE_INFORMATION",
            StatusCode::NO_CONTENT => "NO_CONTENT",
            StatusCode::RESET_CONTENT => "RESET_CONTENT",
            StatusCode::PARTIAL_CONTENT => "PARTIAL_CONTENT",
            StatusCode::MULTI_STATUS => "MULTI_STATUS",
            StatusCode::ALREADY_REPORTED => "ALREADY_REPORTED",
            StatusCode::IM_USED => "IM_USED",
            StatusCode::MULTIPLE_CHOICES => "MULTIPLE_CHOICES",
            StatusCode::MOVED_PERMANENTLY => "MOVED_PERMANENTLY",
            StatusCode::FOUND => "FOUND",
            StatusCode::SEE_OTHER => "SEE_OTHER",
            StatusCode::NOT_MODIFIED => "NOT_MODIFIED",
            StatusCode::USE_PROXY => "USE_PROXY",
            StatusCode::TEMPORARY_REDIRECT => "TEMPORARY_REDIRECT",
            StatusCode::PERMANENT_REDIRECT => "PERMANENT_REDIRECT",
            StatusCode::BAD_REQUEST => "BAD_REQUEST",
            StatusCode::UNAUTHORIZED => "UNAUTHORIZED",
            StatusCode::PAYMENT_REQUIRED => "PAYMENT_REQUIRED",
            StatusCode::FORBIDDEN => "FORBIDDEN",
            StatusCode::NOT_FOUND => "NOT_FOUND",
            StatusCode::METHOD_NOT_ALLOWED => "METHOD_NOT_ALLOWED",
            StatusCode::NOT_ACCEPTABLE => "NOT_ACCEPTABLE",
            StatusCode::PROXY_AUTHENTICATION_REQUIRED => "PROXY_AUTHENTICATION_REQUIRED",
            StatusCode::REQUEST_TIMEOUT => "REQUEST_TIMEOUT",
            StatusCode::CONFLICT => "CONFLICT",
            StatusCode::GONE => "GONE",
            StatusCode::LENGTH_REQUIRED => "LENGTH_REQUIRED",
            StatusCode::PRECONDITION_FAILED => "PRECONDITION_FAILED",
            StatusCode::PAYLOAD_TOO_LARGE => "PAYLOAD_TOO_LARGE",
            StatusCode::URI_TOO_LONG => "URI_TOO_LONG",
            StatusCode::UNSUPPORTED_MEDIA_TYPE => "UNSUPPORTED_MEDIA_TYPE",
            StatusCode::RANGE_NOT_SATISFIABLE => "RANGE_NOT_SATISFIABLE",
            StatusCode::EXPECTATION_FAILED => "EXPECTATION_FAILED",
            StatusCode::IM_A_TEAPOT => "IM_A_TEAPOT",
            StatusCode::MISDIRECTED_REQUEST => "MISDIRECTED_REQUEST",
            StatusCode::UNPROCESSABLE_ENTITY => "UNPROCESSABLE_ENTITY",
            StatusCode::LOCKED => "LOCKED",
            StatusCode::FAILED_DEPENDENCY => "FAILED_DEPENDENCY",
            StatusCode::TOO_EARLY => "TOO_EARLY",
            StatusCode::UPGRADE_REQUIRED => "UPGRADE_REQUIRED",
            StatusCode::PRECONDITION_REQUIRED => "PRECONDITION_REQUIRED",
            StatusCode::TOO_MANY_REQUESTS => "TOO_MANY_REQUESTS",
            StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE => "REQUEST_HEADER_FIELDS_TOO_LARGE",
            StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS => "UNAVAILABLE_FOR_LEGAL_REASONS",
            StatusCode::INTERNAL_SERVER_ERROR => "INTERNAL_SERVER_ERROR",
            StatusCode::NOT_IMPLEMENTED => "NOT_IMPLEMENTED",
            StatusCode::BAD_GATEWAY => "BAD_GATEWAY",
            StatusCode::SERVICE_UNAVAILABLE => "SERVICE_UNAVAILABLE",
            StatusCode::GATEWAY_TIMEOUT => "GATEWAY_TIMEOUT",
            StatusCode::HTTP_VERSION_NOT_SUPPORTED => "HTTP_VERSION_NOT_SUPPORTED",
            StatusCode::VARIANT_ALSO_NEGOTIATES => "VARIANT_ALSO_NEGOTIATES",
            StatusCode::INSUFFICIENT_STORAGE => "INSUFFICIENT_STORAGE",
            StatusCode::LOOP_DETECTED => "LOOP_DETECTED",
            StatusCode::NOT_EXTENDED => "NOT_EXTENDED",
            StatusCode::NETWORK_AUTHENTICATION_REQUIRED => "NETWORK_AUTHENTICATION_REQUIRED",
            _ => unimplemented!("unknown HTTP status code"),
        }
    }

    pub fn as_ident(&self) -> Ident {
        match &self.raw {
            RawStatusCode::Ident(ident) => ident.clone(),
            RawStatusCode::Lit(_) => Ident::new(self.as_text(), Span::call_site()),
        }
    }
}

impl Parse for Status {
    fn parse(input: ParseStream) -> Result<Self> {
        let raw: RawStatusCode = input.parse()?;
        let code: StatusCode = (&raw).try_into()?;

        Ok(Self { raw, code })
    }
}

#[derive(Clone)]
pub enum RawStatusCode {
    Ident(Ident),
    Lit(LitInt),
}

impl TryFrom<&RawStatusCode> for StatusCode {
    type Error = Error;

    fn try_from(value: &RawStatusCode) -> std::result::Result<Self, Self::Error> {
        match value {
            RawStatusCode::Ident(ident) => Ok(match ident.to_string().as_str() {
                "CONTINUE" => StatusCode::CONTINUE,
                "SWITCHING_PROTOCOLS" => StatusCode::SWITCHING_PROTOCOLS,
                "PROCESSING" => StatusCode::PROCESSING,
                "EARLY_HINTS" => StatusCode::EARLY_HINTS,
                "OK" => StatusCode::OK,
                "CREATED" => StatusCode::CREATED,
                "ACCEPTED" => StatusCode::ACCEPTED,
                "NON_AUTHORITATIVE_INFORMATION" => StatusCode::NON_AUTHORITATIVE_INFORMATION,
                "NO_CONTENT" => StatusCode::NO_CONTENT,
                "RESET_CONTENT" => StatusCode::RESET_CONTENT,
                "PARTIAL_CONTENT" => StatusCode::PARTIAL_CONTENT,
                "MULTI_STATUS" => StatusCode::MULTI_STATUS,
                "ALREADY_REPORTED" => StatusCode::ALREADY_REPORTED,
                "IM_USED" => StatusCode::IM_USED,
                "MULTIPLE_CHOICES" => StatusCode::MULTIPLE_CHOICES,
                "MOVED_PERMANENTLY" => StatusCode::MOVED_PERMANENTLY,
                "FOUND" => StatusCode::FOUND,
                "SEE_OTHER" => StatusCode::SEE_OTHER,
                "NOT_MODIFIED" => StatusCode::NOT_MODIFIED,
                "USE_PROXY" => StatusCode::USE_PROXY,
                "TEMPORARY_REDIRECT" => StatusCode::TEMPORARY_REDIRECT,
                "PERMANENT_REDIRECT" => StatusCode::PERMANENT_REDIRECT,
                "BAD_REQUEST" => StatusCode::BAD_REQUEST,
                "UNAUTHORIZED" => StatusCode::UNAUTHORIZED,
                "PAYMENT_REQUIRED" => StatusCode::PAYMENT_REQUIRED,
                "FORBIDDEN" => StatusCode::FORBIDDEN,
                "NOT_FOUND" => StatusCode::NOT_FOUND,
                "METHOD_NOT_ALLOWED" => StatusCode::METHOD_NOT_ALLOWED,
                "NOT_ACCEPTABLE" => StatusCode::NOT_ACCEPTABLE,
                "PROXY_AUTHENTICATION_REQUIRED" => StatusCode::PROXY_AUTHENTICATION_REQUIRED,
                "REQUEST_TIMEOUT" => StatusCode::REQUEST_TIMEOUT,
                "CONFLICT" => StatusCode::CONFLICT,
                "GONE" => StatusCode::GONE,
                "LENGTH_REQUIRED" => StatusCode::LENGTH_REQUIRED,
                "PRECONDITION_FAILED" => StatusCode::PRECONDITION_FAILED,
                "PAYLOAD_TOO_LARGE" => StatusCode::PAYLOAD_TOO_LARGE,
                "URI_TOO_LONG" => StatusCode::URI_TOO_LONG,
                "UNSUPPORTED_MEDIA_TYPE" => StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "RANGE_NOT_SATISFIABLE" => StatusCode::RANGE_NOT_SATISFIABLE,
                "EXPECTATION_FAILED" => StatusCode::EXPECTATION_FAILED,
                "IM_A_TEAPOT" => StatusCode::IM_A_TEAPOT,
                "MISDIRECTED_REQUEST" => StatusCode::MISDIRECTED_REQUEST,
                "UNPROCESSABLE_ENTITY" => StatusCode::UNPROCESSABLE_ENTITY,
                "LOCKED" => StatusCode::LOCKED,
                "FAILED_DEPENDENCY" => StatusCode::FAILED_DEPENDENCY,
                "TOO_EARLY" => StatusCode::TOO_EARLY,
                "UPGRADE_REQUIRED" => StatusCode::UPGRADE_REQUIRED,
                "PRECONDITION_REQUIRED" => StatusCode::PRECONDITION_REQUIRED,
                "TOO_MANY_REQUESTS" => StatusCode::TOO_MANY_REQUESTS,
                "REQUEST_HEADER_FIELDS_TOO_LARGE" => StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
                "UNAVAILABLE_FOR_LEGAL_REASONS" => StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                "INTERNAL_SERVER_ERROR" => StatusCode::INTERNAL_SERVER_ERROR,
                "NOT_IMPLEMENTED" => StatusCode::NOT_IMPLEMENTED,
                "BAD_GATEWAY" => StatusCode::BAD_GATEWAY,
                "SERVICE_UNAVAILABLE" => StatusCode::SERVICE_UNAVAILABLE,
                "GATEWAY_TIMEOUT" => StatusCode::GATEWAY_TIMEOUT,
                "HTTP_VERSION_NOT_SUPPORTED" => StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                "VARIANT_ALSO_NEGOTIATES" => StatusCode::VARIANT_ALSO_NEGOTIATES,
                "INSUFFICIENT_STORAGE" => StatusCode::INSUFFICIENT_STORAGE,
                "LOOP_DETECTED" => StatusCode::LOOP_DETECTED,
                "NOT_EXTENDED" => StatusCode::NOT_EXTENDED,
                "NETWORK_AUTHENTICATION_REQUIRED" => StatusCode::NETWORK_AUTHENTICATION_REQUIRED,
                _ => return Err(Error::new(ident.span(), "invalid HTTP status code")),
            }),
            RawStatusCode::Lit(lit) => StatusCode::from_u16(lit.base10_parse()?)
                .map_err(|_| Error::new(lit.span(), "invalid HTTP status code")),
        }
    }
}

impl Parse for RawStatusCode {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            input.parse().map(RawStatusCode::Ident)
        } else if lookahead.peek(LitInt) {
            input.parse().map(RawStatusCode::Lit)
        } else {
            Err(lookahead.error())
        }
    }
}
