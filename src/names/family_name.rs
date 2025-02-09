use serde::{Deserialize, Serialize};

/// A struct that holds a family name
#[derive(
  Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct FamilyName(String);

impl FamilyName {
  /// Create a new FamilyName
  ///
  /// # Arguments
  /// * `name` - A string slice that holds the name
  pub fn new(name: &str) -> Result<Self, &'static str> {
    if name.is_empty() {
      Err("Family name cannot be empty")
    } else if name.contains(" ") {
      Err("Family name cannot contain spaces")
    } else {
      Ok(FamilyName(name.to_string()))
    }
  }
}

impl TryFrom<&str> for FamilyName {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    if value.is_empty() {
      Err("Family name cannot be empty")
    } else if value.contains(" ") {
      Err("Family name cannot contain spaces")
    } else {
      Ok(FamilyName(value.to_string()))
    }
  }
}

impl std::fmt::Display for FamilyName {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_family_name_new() {
    let family_name = FamilyName::new("Smith").unwrap();
    assert_eq!(family_name.0, "Smith");
  }

  #[test]
  fn test_family_name_try_from() {
    let family_name = FamilyName::try_from("Smith").unwrap();
    assert_eq!(family_name.0, "Smith");
  }
}
