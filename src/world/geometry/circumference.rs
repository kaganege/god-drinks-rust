use crate::world::{thing, ToAttribute};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Circumference(u32);

impl ToAttribute for Circumference {
  fn to_attribute(&self) -> thing::Attribute {
    thing::Attribute::new_with_value("circumference", self.0.to_string())
  }
}
