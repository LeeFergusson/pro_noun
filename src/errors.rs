#[derive(Debug)]
pub enum Error {
  DynError(Box<dyn std::error::Error>),
  ProNounError(String),
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Error::DynError(error) => write!(f, "{}", error),
      Error::ProNounError(error) => write!(f, "{}", error),
    }
  }
}

impl std::error::Error for Error {}

impl From<Box<dyn std::error::Error>> for Error {
  fn from(error: Box<dyn std::error::Error>) -> Self {
    Error::DynError(error)
  }
}

impl From<ProNounError> for Error {
  fn from(error: ProNounError) -> Self {
    Error::ProNounError(format!("{:?}", error))
  }
}

impl From<&str> for Error {
  fn from(error: &str) -> Self {
    Error::ProNounError(error.to_string())
  }
}

#[derive(Debug)]
pub enum ProNounError {}

// pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = core::result::Result<T, Error>;
