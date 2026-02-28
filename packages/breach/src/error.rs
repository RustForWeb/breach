use http::StatusCode;

/// HTTP error.
pub trait HttpError {
    /// HTTP status code.
    fn status(&self) -> StatusCode;

    /// Hook called when the HTTP error is used as response.
    fn hook(&self);
}
