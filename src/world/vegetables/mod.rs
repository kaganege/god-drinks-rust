mod eggplant;
mod tomato;

pub use eggplant::Eggplant;
pub use tomato::Tomato;

use super::{thing::Attribute, ToAttribute};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Antioxidant;

impl ToAttribute for Antioxidant {
  fn to_attribute(&self) -> Attribute {
    Attribute::new("antioxidant")
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Nutrient;

impl ToAttribute for Nutrient {
  fn to_attribute(&self) -> Attribute {
    Attribute::new("nutrient")
  }
}
