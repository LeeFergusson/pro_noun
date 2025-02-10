//! This module contains the Psydonym struct and its implementation.
//! A Psydonym is a name that is an alias for another name.

// -- External Modules
use serde::{Deserialize, Serialize};

// -- Internal Modules
use super::{NameTrait, PersonsName};

// -- Psuedonym Stucture
#[derive(
  Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Psydonym {
  alias_for: PersonsName,
  name: PersonsName,
}

// -- Implement Psydonym
impl Psydonym {
  /// Creates a new Psydonym instance.
  ///
  /// # Arguments
  /// * `alias_for` - A reference to the CharacterName that the Psydonym is an alias for.
  /// * `name` - A reference to the CharacterName that is the alias.
  pub fn new(alias_for: &PersonsName, name: &PersonsName) -> Self {
    Psydonym {
      alias_for: alias_for.clone(),
      name: name.clone(),
    }
  }

  /// Returns the CharacterName that the Psydonym is an alias for.
  ///
  /// # Returns
  /// A reference to the CharacterName that the Psydonym is an alias for.
  pub fn alias_for(&self) -> &PersonsName {
    &self.alias_for
  }

  /// Returns the CharacterName that is the alias.
  ///
  /// # Returns
  /// A reference to the CharacterName that is the alias.
  pub fn name(&self) -> Option<String> {
    Some(self.name.to_string())
  }

  /// Returns the true name of the character.
  ///
  /// # Returns
  /// A reference to the CharacterName that is the true name of the character.
  pub fn true_name(&self) -> &PersonsName {
    &self.alias_for
  }
}

// -- Implement Display for Psydonym
impl std::fmt::Display for Psydonym {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}

// -- Implement NameTrait for Psydonym
impl NameTrait for Psydonym {
  fn full_name(&self) -> String {
    self.name.full_name()
  }

  fn given_name(&self) -> Option<String> {
    if let Some(given_name) = self.name.given_name() {
      Some(given_name.to_string())
    } else {
      None
    }
  }

  fn family_name(&self) -> Option<String> {
    if let Some(family_name) = self.name.family_name() {
      Some(family_name.to_string())
    } else {
      None
    }
  }
}

// -- Tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_with_valid_values() {
    let name = PersonsName::new().with_given_name("Olórin").unwrap();
    let alias = PersonsName::new().with_given_name("Gandalf").unwrap();

    let psuedonym = Psydonym::new(&name, &alias);

    assert_eq!(psuedonym.alias_for(), &name);
    assert_eq!(psuedonym.name(), Some("Gandalf".to_string()));
  }

  #[test]
  fn full_name() {
    let name = PersonsName::new().with_given_name("Olórin").unwrap();
    let alias = PersonsName::new().with_given_name("Gandalf").unwrap();

    let psuedonym = Psydonym::new(&name, &alias);

    assert_eq!(psuedonym.full_name(), "Gandalf".to_string());
  }

  #[test]
  fn true_name() {
    let name = PersonsName::new().with_given_name("Olórin").unwrap();
    let alias = PersonsName::new().with_given_name("Gandalf").unwrap();

    let psuedonym = Psydonym::new(&name, &alias);

    assert_eq!(psuedonym.true_name(), &name);
  }

  #[test]
  fn name_trait_given_name() {
    let name = PersonsName::new().with_given_name("Olórin").unwrap();
    let alias = PersonsName::new().with_given_name("Gandalf").unwrap();

    let psuedonym = Psydonym::new(&name, &alias);

    assert_eq!(psuedonym.given_name(), Some("Gandalf".to_string()));
  }

  #[test]
  fn name_trait_family_name() {
    let name = PersonsName::new().with_family_name("Mithrandir").unwrap();
    let alias = PersonsName::new().with_family_name("Greyhame").unwrap();

    let psuedonym = Psydonym::new(&name, &alias);

    assert_eq!(psuedonym.family_name(), Some("Greyhame".to_string()));
  }

  #[test]
  fn name_trait_full_name() {
    let name = PersonsName::new().with_given_name("Olórin").unwrap();
    let alias = PersonsName::new().with_given_name("Gandalf").unwrap();

    let psuedonym = Psydonym::new(&name, &alias);

    assert_eq!(psuedonym.full_name(), "Gandalf".to_string());
  }
}
