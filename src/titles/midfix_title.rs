// -- Internal Dependencies
use crate::Error;

// -- MidfixTitle Structure

/// A struct that holds a midfix title
///
/// # Fields
/// * `title` - A string slice that holds the title
pub struct MidfixTitle {
  title: String,
}

impl MidfixTitle {
  /// Create a new MidfixTitle
  ///
  /// # Arguments
  /// * `title` - A string slice that holds the title
  ///
  /// # Returns
  /// A Result containing the MidfixTitle or an Error
  ///
  /// # Errors
  /// * If the title is empty
  pub fn new(title: &str) -> Result<Self, Error> {
    if title.is_empty() {
      Err(Error::from("Midfix title cannot be empty"))
    } else {
      Ok(Self {
        title: title.to_string(),
      })
    }
  }

  /// Get the name of the MidfixTitle
  pub fn title(&self) -> &str {
    &self.title
  }
}

// -- Implement the TryFrom trait for MidfixTitle
impl TryFrom<&str> for MidfixTitle {
  type Error = Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    if value.is_empty() {
      Err(Error::from("Midfix title cannot be empty"))
    } else {
      Ok(Self {
        title: value.to_string(),
      })
    }
  }
}

// -- Implement the Display trait for MidfixTitle
impl std::fmt::Display for MidfixTitle {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.title)
  }
}

// -- Tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_midfix_title() {
    let title = MidfixTitle::new("Dr.").unwrap();
    assert_eq!(title.title(), "Dr.");
  }

  #[test]
  fn try_from_string_slice_to_midfix_title() {
    let title = MidfixTitle::try_from("Dr.").unwrap();
    assert_eq!(title.title(), "Dr.");
  }

  #[test]
  fn new_midfix_title_empty_should_fail() {
    let title = MidfixTitle::new("");
    assert!(title.is_err());
  }

  #[test]
  fn try_from_midfix_title_empty_should_fail() {
    let title = MidfixTitle::try_from("");
    assert!(title.is_err());
  }
}
