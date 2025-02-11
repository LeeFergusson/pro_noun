//! Represents a prefix title.

use serde::{Deserialize, Serialize};

use crate::Error;

/// A struct that holds a prefix title
#[derive(
  Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct PrefixTitle {
  name: String,
  abbreviation: Option<String>,
}

impl PrefixTitle {
  /// Create a new PrefixTitle
  ///
  /// # Arguments
  /// * `title` - A string slice that holds the title
  ///
  /// # Returns
  /// A Result containing the PrefixTitle or an Error
  ///
  /// # Errors
  /// * If the title is empty
  pub fn new(title: &str) -> Result<Self, Error> {
    if title.is_empty() {
      Err(Error::from("Prefix title cannot be empty"))
    } else {
      Ok(Self {
        name: title.to_string(),
        abbreviation: None,
      })
    }
  }

  /// Set the abbreviation for the PrefixTitle
  ///
  /// # Arguments
  /// * `abbreviation` - A string slice that holds the abbreviation
  ///
  /// # Returns
  /// A Result containing the PrefixTitle or an Error
  pub fn with_abbreviation(self, abbreviation: &str) -> Result<Self, Error> {
    Ok(Self {
      abbreviation: Some(abbreviation.to_string()),
      ..self
    })
  }

  /// Get the name of the PrefixTitle
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Get the name of the PrefixTitle
  pub fn abbreviation(&self) -> Option<&str> {
    self.abbreviation.as_deref()
  }
}

// -- Implement the TryFrom trait for PrefixTitle
impl TryFrom<&str> for PrefixTitle {
  type Error = Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    if value.is_empty() {
      Err(Error::from("Prefix title cannot be empty"))
    } else {
      Ok(Self {
        name: value.to_string(),
        abbreviation: None,
      })
    }
  }
}

// -- Implement Display for PrefixTitle
impl std::fmt::Display for PrefixTitle {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}

// -- Tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_prefix_title() {
    let title = PrefixTitle::new("Doctor").unwrap();

    assert_eq!(title.name(), "Doctor");
    assert_eq!(title.abbreviation(), None);
  }

  #[test]
  fn create_prefix_title_with_abbreviation() {
    let title = PrefixTitle::new("Doctor")
      .unwrap()
      .with_abbreviation("Dr")
      .unwrap();

    assert_eq!(title.name(), "Doctor");
    assert_eq!(title.abbreviation(), Some("Dr"));
  }

  #[test]
  fn try_from_string() {
    let title = PrefixTitle::try_from("Doctor").unwrap();
    assert_eq!(title.name(), "Doctor");
    assert_eq!(title.abbreviation(), None);
  }
}
