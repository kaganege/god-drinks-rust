use std::fmt::Display;

use crate::world::{thing, ToAttribute};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Dimensions {
  OneD,
  TwoD,
  ThreeD,
}

impl Display for Dimensions {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Dimensions::OneD => write!(f, "1D"),
      Dimensions::TwoD => write!(f, "2D"),
      Dimensions::ThreeD => write!(f, "3D"),
    }
  }
}

impl ToAttribute for Dimensions {
  fn to_attribute(&self) -> thing::Attribute {
    thing::Attribute::new_with_value("dimensions", self.to_string())
  }
}
