use http::StatusCode;

/// HTTP error.
pub trait HttpError {
    /// HTTP status code.
    fn status(&self) -> StatusCode;
}
