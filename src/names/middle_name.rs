//! Middle name module
//!
//! This module contains the implementation of the MiddleName structure.

/// MiddleName structure
#[derive(
  Debug,
  Clone,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Hash,
  serde::Serialize,
  serde::Deserialize,
)]
pub struct MiddleName(String);

// -- Implement MiddleName structure
impl MiddleName {
  pub fn new(value: &str) -> Result<Self, String> {
    if value.is_empty() {
      Err("Middle name cannot be empty".to_string())
    } else {
      Ok(Self(value.to_string()))
    }
  }
}

// -- Implement TryFrom trait for MiddleName
impl TryFrom<&str> for MiddleName {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    MiddleName::new(value)
  }
}

// -- Implement Display trait for MiddleName
impl std::fmt::Display for MiddleName {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

// -- Tests

#[cfg(test)]
mod tests {
  use super::*;

  // --- MiddleName tests

  // Should create a new MiddleName instance
  #[test]
  fn middle_name_new() {
    let middle_name = MiddleName::new("John");
    assert_eq!(middle_name.unwrap().to_string(), "John");
  }

  // Should be able to create a new MiddleName instance using TryFrom
  #[test]
  fn middle_name_try_from() {
    let middle_name = MiddleName::try_from("John");
    assert_eq!(middle_name.unwrap().to_string(), "John");
  }

  // Should not be able to create a new MiddleName instance with an empty value
  #[test]
  fn middle_name_try_from_empty() {
    let middle_name = MiddleName::try_from("");
    assert_eq!(middle_name, Err("Middle name cannot be empty".to_string()));
  }

  // Should display the MiddleName correctly
  #[test]
  fn middle_name_display() {
    let middle_name = MiddleName::new("John").unwrap();
    assert_eq!(middle_name.to_string(), "John");
  }
}
