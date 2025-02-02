use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
  Submissive,
  Dominant,
}

impl Display for Role {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Role::Submissive => write!(f, "submissive"),
      Role::Dominant => write!(f, "dominant"),
    }
  }
}

impl Role {
  pub fn toggle(&self) -> Self {
    match self {
      Role::Submissive => Role::Dominant,
      Role::Dominant => Role::Submissive,
    }
  }
}
