//! CharacterName module.
//! This module contains the CharacterName struct.

// -- External Modules
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Error;

// -- Project Modules
use super::{FamilyName, GivenName, MiddleName, Psydonym};

/// The name of a person.
/// A personal name is composed of a given name and an optional family name.
/// A personal name may also contain one or more middle names.
#[derive(
  Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct PersonalName {
  given_name: Option<GivenName>,
  middle_names: Vec<MiddleName>,
  family_name: Option<FamilyName>,
}

// -- Implement Default for PersonalName
impl Default for PersonalName {
  fn default() -> Self {
    PersonalName {
      given_name: None,
      middle_names: Vec::new(),
      family_name: None,
    }
  }
}

// -- Implement Display for PersonalName
impl std::fmt::Display for PersonalName {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match &self.family_name {
      Some(family_name) => match &self.given_name {
        Some(given_name) => write!(f, "{} {}", given_name, family_name),
        None => write!(f, "{}", family_name),
      },
      None => match &self.given_name {
        Some(given_name) => write!(f, "{}", given_name),
        None => write!(f, "Unknown"),
      },
    }
  }
}

// -- Implement CharacterName
impl PersonalName {
  /// Creates a new CharacterName instance.
  ///
  /// # Returns
  /// A CharacterName instance.
  pub fn new() -> Self {
    PersonalName {
      given_name: None,
      middle_names: Vec::new(),
      family_name: None,
    }
  }

  /// Returns the GivenName
  pub fn given_name(&self) -> Option<&GivenName> {
    self.given_name.as_ref()
  }

  pub fn middle_names(&self) -> &Vec<MiddleName> {
    &self.middle_names
  }

  /// Returns the FamilyName
  ///
  /// # Returns
  /// An Option containing the FamilyName
  pub fn family_name(&self) -> Option<&FamilyName> {
    self.family_name.as_ref()
  }

  /// Sets the given name
  ///
  /// # Arguments
  /// * `name` - A string slice that holds the name
  ///
  /// # Returns
  /// A Result containing the CharacterName or an Error
  pub fn with_given_name(self, name: &str) -> Result<Self, Error> {
    let given_name = GivenName::new(name)?;
    Ok(PersonalName {
      given_name: Some(given_name),
      // middle_names: self.middle_names,
      // family_name: self.family_name,
      ..self
    })
  }

  pub fn with_middle_name(self, name: &str) -> Result<Self, Error> {
    let middle_name = MiddleName::new(name)?;
    let mut middle_names = self.middle_names;
    middle_names.push(middle_name);
    Ok(PersonalName {
      given_name: self.given_name,
      middle_names,
      family_name: self.family_name,
    })
  }

  /// Sets the family name
  ///
  /// # Arguments
  /// * `name` - A string slice that holds the name
  ///
  /// # Returns
  /// A Result containing the CharacterName or an Error
  pub fn with_family_name(self, name: &str) -> Result<Self, Error> {
    let family_name = FamilyName::new(name)?;
    Ok(PersonalName {
      // given_name: self.given_name,
      // middle_names: self.middle_names,
      family_name: Some(family_name),
      ..self
    })
  }

  pub fn full_name(&self) -> String {
    let mut full_name = String::new();
    if let Some(given_name) = &self.given_name {
      full_name.push_str(given_name.to_string().as_str());
    }
    for middle_name in &self.middle_names {
      full_name.push_str(" ");
      full_name.push_str(middle_name.to_string().as_str());
    }
    if let Some(family_name) = &self.family_name {
      full_name.push_str(" ");
      full_name.push_str(family_name.to_string().as_str());
    }
    full_name
  }

  pub fn full_name_short(&self) -> String {
    let mut full_name = String::new();
    if let Some(given_name) = &self.given_name {
      full_name.push_str(given_name.to_string().as_str());
    }
    for middle_name in &self.middle_names {
      full_name.push_str(" ");
      full_name.push_str(
        middle_name
          .to_string()
          .chars()
          .next()
          .unwrap()
          .to_string()
          .as_str(),
      );
      full_name.push_str(".");
    }
    if let Some(family_name) = &self.family_name {
      full_name.push_str(" ");
      full_name.push_str(family_name.to_string().as_str());
    }
    full_name
  }

  /// Converts the CharacterName to a JSON Value
  ///
  /// # Returns
  /// A JSON Value representing the CharacterName
  pub fn to_json(&self) -> Value {
    serde_json::json!({
      "given_name": self.given_name,
      "middle_name": self.middle_names,
      "family_name": self.family_name,
    })
  }
}

pub enum Name {
  TrueName(PersonalName),
  Psuedonym(Psydonym),
}

pub trait NameTrait {
  /// Returns the full name of the name
  ///
  /// # Returns
  /// A string containing the full name
  fn full_name(&self) -> String;

  /// Returns the given name of the name
  ///
  /// # Returns
  /// A string containing the given name or None
  fn given_name(&self) -> Option<String>;

  /// Returns the family name of the name
  ///
  /// # Returns
  /// A string containing the family name or None
  fn family_name(&self) -> Option<String>;
}

// -- Implement NameTrait
impl NameTrait for Name {
  fn full_name(&self) -> String {
    match self {
      Name::TrueName(name) => name.full_name(),
      Name::Psuedonym(psydonym) => psydonym.full_name(),
    }
  }

  fn given_name(&self) -> Option<String> {
    match self {
      Name::TrueName(name) => name.given_name().map(|name| name.to_string()),
      Name::Psuedonym(psydonym) => psydonym.given_name(),
    }
  }

  fn family_name(&self) -> Option<String> {
    match self {
      Name::TrueName(name) => name.family_name().map(|name| name.to_string()),
      Name::Psuedonym(psydonym) => psydonym.family_name(),
    }
  }
}
