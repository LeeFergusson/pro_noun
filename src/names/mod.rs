//! Name module
//!
//! This module contains the name related functionality for the pathfinder library.

// --- Modules
mod family_name;
mod given_name;
mod middle_name;
mod persons_name;
mod psydonym;

// -- Flatern the module structure
pub use family_name::*;
pub use given_name::*;
pub use middle_name::*;
pub use persons_name::*;
pub use psydonym::*;
