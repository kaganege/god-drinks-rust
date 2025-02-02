use super::{Antioxidant, Nutrient};

pub struct Eggplant {
  nutrients: Vec<Nutrient>,
  antioxidants: Vec<Antioxidant>,
}

crate::impl_thing!(Eggplant);

impl Eggplant {
  pub fn new() -> Self {
    Self {
      nutrients: Vec::new(),
      antioxidants: Vec::new(),
    }
  }

  pub fn nutrients(&self) -> &[Nutrient] {
    &self.nutrients
  }

  pub fn add_nutrient(&mut self, nutrient: Nutrient) {
    self.nutrients.push(nutrient);
  }

  pub fn reset_nutrients(&mut self) {
    self.nutrients.clear();
  }

  pub fn antioxidants(&self) -> &[Antioxidant] {
    &self.antioxidants
  }

  pub fn add_antioxidant(&mut self, antioxidant: Antioxidant) {
    self.antioxidants.push(antioxidant);
  }

  pub fn reset_antioxidants(&mut self) {
    self.antioxidants.clear();
  }
}
