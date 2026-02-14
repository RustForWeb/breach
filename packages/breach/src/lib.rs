#![warn(missing_docs)]

//! Breach.

mod error;
#[cfg(feature = "utoipa")]
pub mod utoipa;

pub use error::*;

#[cfg(feature = "macros")]
pub use breach_macros::*;
pub use http;
