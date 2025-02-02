use super::{Antioxidant, Nutrient};

#[derive(Default)]
pub struct Tomato {
  nutrients: Vec<Nutrient>,
  antioxidants: Vec<Antioxidant>,
}

impl Tomato {
  pub fn new() -> Self {
    Self::default()
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
