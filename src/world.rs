#![allow(dead_code)]

use std::fmt::Display;

use lockable::{Lockable, MakeLockable};
use lovable::Lovable;

pub use execution::{Execution, ToExecution};
pub use message::Message;
pub use thing::{Thing, ToAttribute};

mod execution;
mod lockable;
mod message;
mod thing;

pub mod animals;
pub mod geometry;
pub mod lovable;
pub mod vegetables;

#[derive(Default)]
pub struct World {
  pub things: Vec<MakeLockable<Box<dyn Thing>>>,

  god: Option<Lovable>,
}

impl Display for World {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "the world")
  }
}

impl World {
  pub fn new(_: u32) -> Self {
    Self::default()
  }

  pub fn start_simulation(&self) {
    println!("Starting simulation");
  }

  pub fn add_thing(&mut self, thing: Box<dyn Thing>) {
    println!("Added {:?} to the world", thing.name());

    if let Some(lovable) = self
      .god
      .as_ref()
      .or_else(|| thing.as_any().downcast_ref::<Lovable>())
    {
      self.god = Some(lovable.clone());
    }

    self.things.push(MakeLockable::new(thing));
  }

  pub fn remove_thing(&mut self, thing: &impl Thing) {
    if self.god.as_ref().is_some_and(|g| g.id() == thing.id()) {
      panic!("God can't be removed!!!");
    }

    self.things.retain(|t| t.id() != thing.id());

    println!("{:?} is gone from the world", thing.name());
  }

  pub fn thing_index(&self, thing: &impl Thing) -> Option<usize> {
    self.things.iter().position(|t| t.id() == thing.id())
  }

  pub fn god(&self) -> &Lovable {
    self.god.as_ref().expect("You have no god!")
  }

  pub fn lock_thing(&mut self, thing: &impl Thing) {
    println!("Locking {}", thing.name());

    self
      .things
      .iter_mut()
      .find(|t| t.id() == thing.id())
      .unwrap()
      .lock();
  }
  pub fn unlock(&mut self, thing: &impl Thing) {
    println!("Unlocking {}", thing.name());

    self
      .things
      .iter_mut()
      .find(|t| t.id() == thing.id())
      .unwrap()
      .unlock();
  }

  pub fn announce(&self, message: impl Message) {
    println!("Announcement: {}", message.display());
  }

  pub fn time_travel_for_two(&self, _: &str, _: i32, _: &impl Thing, _: &impl Thing) {}
  pub fn unite(&self, _: &impl Thing, _: &impl Thing) {}
  pub fn procreate(&self, thing1: &impl Thing, thing2: &impl Thing) {
    println!(
      "{:?} and {:?} doing {}",
      thing1.name(),
      thing2.name(),
      "\u{FFFD}".repeat(5)
    );
  }
  pub fn make_high(&self, thing: &impl Thing) {
    println!("{:?} is high now", thing.name());
  }

  pub fn is_executable_by(&self, thing: &impl Thing) -> bool {
    let can_execute = thing.id() == self.god.as_ref().unwrap().id();

    println!(
      "The world is{} executable by {:?}",
      if can_execute { "" } else { " not" },
      thing.name()
    );

    can_execute
  }

  pub fn run_execution(&self) {
    println!("EXECUTION");
  }

  pub fn execute(&self, thing: impl Thing) {
    println!("Executing {}", thing.name());
  }
}
