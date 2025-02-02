use std::fmt::Display;

#[derive(Clone, PartialEq, Eq)]
pub struct Proof(pub String);

impl Display for Proof {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
