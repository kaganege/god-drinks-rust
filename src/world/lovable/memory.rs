#[derive(Default, Clone, PartialEq, Eq)]
pub struct Memory {
  data: Vec<u8>,
  erasable: bool,
}

impl Memory {
  pub fn new(erasable: bool) -> Self {
    Self {
      erasable,
      ..Default::default()
    }
  }

  pub fn is_erasable(&self) -> bool {
    self.erasable
  }
}
