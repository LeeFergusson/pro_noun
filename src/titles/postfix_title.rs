//! The postfix_title Module

// -- External Dependencies
use serde::{Deserialize, Serialize};

// -- Internal Dependencies
use crate::Error;

// -- PostfixTitle Structure

/// A struct that holds a postfix title
///
/// # Fields
/// * `name` - A string slice that holds the name
/// * `abbreviation` - An optional string slice that holds the abbreviation
#[derive(
  Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct PostfixTitle {
  name: String,
  abbreviation: Option<String>,
}

// -- Implement PostfixTitle
impl PostfixTitle {
  /// Create a new PostfixTitle
  ///
  /// # Arguments
  /// * `title` - A string slice that holds the title
  ///
  /// # Returns
  /// A Result containing the PostfixTitle or an Error
  pub fn new(title: &str) -> Result<Self, Error> {
    if title.is_empty() {
      Err(Error::from("Postfix title cannot be empty"))
    } else {
      Ok(Self {
        name: title.to_string(),
        abbreviation: None,
      })
    }
  }

  /// Set the abbreviation for the PostfixTitle
  ///
  /// # Arguments
  /// * `abbreviation` - A string slice that holds the abbreviation
  ///
  /// # Returns
  /// A Result containing the PostfixTitle or an Error
  pub fn with_abbreviation(self, abbreviation: &str) -> Result<Self, Error> {
    if abbreviation.is_empty() {
      Err(Error::from("Postfix title abbreviation cannot be empty"))
    } else {
      Ok(Self {
        abbreviation: Some(abbreviation.to_string()),
        ..self
      })
    }
  }

  /// Get the name of the PostfixTitle
  ///
  /// # Returns
  /// A string slice containing the name
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Get the abbreviation of the PostfixTitle
  ///
  /// # Returns
  /// An optional string slice containing the abbreviation
  pub fn abbreviation(&self) -> Option<&str> {
    self.abbreviation.as_deref()
  }
}

// -- Implment TryFrom<&str> for PostfixTitle
impl TryFrom<&str> for PostfixTitle {
  type Error = Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Self::new(value)
  }
}

// -- Tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_postfix_title() {
    let title = PostfixTitle::new("PhD").unwrap();
    assert_eq!(title.name(), "PhD");
    assert_eq!(title.abbreviation(), None);
  }

  #[test]
  fn new_postfix_title_with_abbreviation() {
    let title = PostfixTitle::new("PhD")
      .unwrap()
      .with_abbreviation("Dr.")
      .unwrap();
    assert_eq!(title.name(), "PhD");
    assert_eq!(title.abbreviation(), Some("Dr."));
  }

  #[test]
  fn new_postfix_title_empty_should_fail() {
    let title = PostfixTitle::new("");
    assert!(title.is_err());
  }

  #[test]
  fn new_postfix_title_with_abbreviation_empty_should_fail() {
    let title = PostfixTitle::new("PhD").unwrap().with_abbreviation("");
    assert!(title.is_err());
  }

  #[test]
  fn try_from_string_slice() {
    let title = PostfixTitle::try_from("Doctor of Philosophy").unwrap();
    assert_eq!(title.name(), "Doctor of Philosophy");
    assert_eq!(title.abbreviation(), None);
  }
}
