pub struct TabbyCat;

crate::impl_thing!(TabbyCat);

impl TabbyCat {
  pub fn purr(&self) {
    println!("Purr");
  }
}
