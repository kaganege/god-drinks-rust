#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
  Male,
  Female,
}

impl Gender {
  pub fn toggle(&self) -> Self {
    match self {
      Gender::Male => Gender::Female,
      Gender::Female => Gender::Male,
    }
  }
}
