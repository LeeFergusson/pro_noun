//! # The pro_noun Module
//!
//! This module contains types and functions for managing names.

// -- Modules
mod errors;
mod names;
mod titles;

// -- Flatern the module structure
pub use errors::{Error, Result};
pub use names::*;
pub use titles::*;
