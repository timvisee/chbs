//! Convenience re-export of common members
//!
//! Like the standard library's prelude, this module simplifies importing of
//! common items such as traits. Unlike the standard prelude, the contents of
//! this module must be imported manually.
//!
//! # Examples
//! ```rust
//! extern crate chbs;
//! use chbs::{config::BasicConfig, prelude::*};
//!
//! let config = BasicConfig::default();
//!
//! // This method requires the ToScheme trait, imported through prelude
//! let scheme = config.to_scheme();
//! ```

// Prelude common traits
pub use component::traits::*;
pub use entropy::HasEntropy;
pub use scheme::ToScheme;
