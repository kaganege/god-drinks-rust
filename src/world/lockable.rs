use std::ops::Deref;

pub trait Lockable {
  fn is_locked(&self) -> bool;
  fn lock(&mut self);
  fn unlock(&mut self);
}

pub struct MakeLockable<T> {
  thing: T,
  locked: bool,
}

impl<T> MakeLockable<T> {
  pub fn new(thing: T) -> Self {
    Self {
      thing,
      locked: false,
    }
  }
}

impl<T> Deref for MakeLockable<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.thing
  }
}

impl<T> Lockable for MakeLockable<T> {
  fn is_locked(&self) -> bool {
    self.locked
  }

  fn lock(&mut self) {
    self.locked = true;
  }

  fn unlock(&mut self) {
    self.locked = false;
  }
}
