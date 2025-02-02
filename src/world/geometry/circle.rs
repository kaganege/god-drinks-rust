use super::Circumference;

pub struct Circle {
  #[allow(dead_code)]
  circumference: Circumference,
}

impl Circle {
  pub fn circumference(&self) -> Circumference {
    self.circumference
  }

  pub fn reset_circumference(&mut self) {
    self.circumference = Default::default();
  }
}
