use super::Dimensions;

pub struct PointSet {
  dimensions: Dimensions,
}

impl PointSet {
  pub fn dimensions(&self) -> Dimensions {
    self.dimensions
  }

  pub fn reset_dimensions(&mut self) {
    self.dimensions = Dimensions::OneD;
  }
}
