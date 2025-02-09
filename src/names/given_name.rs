//! This module contains the GivenName struct and its implementations.
//!
//! A GivenName is a struct that holds a given name for a character.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Error;

/// A struct that holds a given name
#[derive(
  Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct GivenName(String);

impl GivenName {
  /// Create a new GivenName
  ///
  /// # Arguments
  /// * `name` - A string slice that holds the name
  ///
  /// # Returns
  /// A Result containing the GivenName or an Error
  pub fn new(name: &str) -> Result<Self, Error> {
    if name.is_empty() {
      Err(Error::from("Given name cannot be empty"))
    } else if name.contains(" ") {
      Err(Error::from("Given name cannot contain spaces"))
    } else {
      Ok(GivenName(name.to_string()))
    }
  }

  /// Convert the GivenName to a JSON Value
  pub fn to_json(&self) -> Value {
    serde_json::json!(self.0)
  }
}

// -- Implement the Default trait for GivenNames
impl Default for GivenName {
  fn default() -> Self {
    GivenName("Unknown".to_string())
  }
}

// -- Implement the TryFrom trait for GivenName
impl TryFrom<&str> for GivenName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    if value.is_empty() {
      Err("cannot be empty")
    } else if value.contains(" ") {
      Err("cannot contain spaces")
    } else {
      Ok(GivenName(value.to_string()))
    }
  }
}

// Implement the Display trait for GivenName
impl std::fmt::Display for GivenName {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /// Should create a new GivenName given valid values
  #[test]
  fn new_creates_name_with_vald_values() {
    let given_name = GivenName::new("John").unwrap();

    assert_eq!(
      given_name,
      GivenName {
        0: "John".to_string()
      }
    )
  }

  // Should return an error when the name is empty
  #[test]
  fn new_errors_when_empty() {
    let given_name = GivenName::new("");
    assert_eq!(given_name.is_err(), true);
  }

  #[test]
  fn correct_default_name() {
    let given_name = GivenName::default();
    assert_eq!(given_name.0, "Unknown");
  }
}
