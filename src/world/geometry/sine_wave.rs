pub struct SineWave {
  #[allow(dead_code)]
  frequency: u32,
}

crate::impl_thing!(SineWave);

impl SineWave {
  pub fn tangent(&self, _source: u32) -> (u32, u32) {
    todo!()
  }
}
