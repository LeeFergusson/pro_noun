//! # The pro_noun Module
//!
//! This module contains types and functions for managing names.

// -- Modules
mod errors;
mod names;

// -- Flatern the module structure
pub use errors::{Error, Result};
pub use names::*;
