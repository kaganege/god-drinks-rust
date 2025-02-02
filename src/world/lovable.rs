use std::{
  fmt::Display,
  hash::{DefaultHasher, Hash, Hasher},
};

use super::{thing::Current, Execution, Thing, ToExecution, World};

pub use gender::Gender;
pub use memory::Memory;
pub use opinion::Opinion;
pub use role::Role;

mod gender;
mod memory;
mod opinion;
mod role;

#[derive(Debug)]
pub enum OpinionError {
  IllegalArgument,
  NotFound,
}

#[derive(Clone)]
pub struct Lovable {
  name: &'static str,
  current: Current,
  can_see: Option<bool>,
  feelings: Vec<String>,
  satisfaction: u32,
  gender: Gender,
  role: Role,
  senses: Vec<String>,
  memory: Memory,
  opinions: Vec<Opinion<String, bool>>,
  execution: Execution,
}

crate::impl_thing! {
  Lovable {
    fn id(&self) -> u64 {
      let mut hasher = DefaultHasher::new();
      self.name.hash(&mut hasher);
      hasher.finish()
    }

    fn name(&self) -> &str {
      self.name
    }
  }
}

impl Lovable {
  pub fn new(name: &'static str, _: u32, gender: bool, _: i32, _: bool) -> Self {
    let gender = if gender { Gender::Female } else { Gender::Male };

    Self {
      name,
      gender,
      feelings: Default::default(),
      satisfaction: Default::default(),
      can_see: Default::default(),
      current: Default::default(),
      role: match gender {
        Gender::Male => Role::Dominant,
        Gender::Female => Role::Submissive,
      },
      senses: Default::default(),
      memory: Memory::new(true),
      opinions: vec![Opinion::new("you are here", true)],
      execution: Execution,
    }
  }

  pub fn toggle_current(&mut self) {
    println!(
      "{:?}s current is changed from {:?} to {:?}",
      self.name,
      self.current,
      self.current.toggle()
    );

    self.current = self.current.toggle();
  }

  pub fn set_can_see(&mut self, can_see: bool) {
    if self.can_see.map(|p| p != can_see).unwrap_or(true) {
      println!(
        "{:?} {} see now",
        self.name,
        if can_see { "can" } else { "can't" }
      );
    }

    self.can_see = Some(can_see);
  }

  pub fn add_feeling(&mut self, feeling: &str) {
    println!("Adding {} to {}", feeling, self.name);

    self.feelings.push(feeling.to_string());
  }

  pub fn remove_feeling(&mut self, feeling: &str) {
    println!("Removing {} from {}", feeling, self.name);

    self.feelings.retain(|f| f != feeling);
  }

  pub fn feeling_index(&self, feeling: &str) -> Option<usize> {
    self.feelings.iter().position(|f| f == feeling)
  }

  pub fn set_satisfaction(&mut self, satisfaction: u32) {
    self.satisfaction = satisfaction;
  }

  pub fn to_satisfaction(&self) -> u32 {
    self.satisfaction
  }

  pub fn toggle_gender(&mut self) {
    println!(
      "{:?}s gender is changed from {:?} to {:?}",
      self.name,
      self.gender,
      self.gender.toggle()
    );

    self.gender = self.gender.toggle();
  }

  pub fn toggle_role_bdsm(&mut self) {
    println!("{:?} is now {}", self.name, self.role.toggle());

    self.role = self.role.toggle();
  }

  pub fn senses(&self) -> &[String] {
    &self.senses
  }

  pub fn add_sense(&mut self, sense: &str) {
    self.senses.push(sense.to_string());
  }

  pub fn reset_senses(&mut self) {
    self.senses.clear();
  }

  pub fn sense_index(&self, sense: &str) -> Option<usize> {
    self.senses.iter().position(|s| s == sense)
  }

  pub fn look_for(&self, thing: &impl Thing, environment: &World) -> bool {
    let can_see = self.can_see.unwrap_or_default();

    if !can_see {
      println!("{:?} can't see!", self.name);
    }

    can_see
      && match environment.thing_index(thing) {
        Some(_) => {
          println!("{:?} is in the environment", thing.name());
          true
        }

        None => {
          println!("{:?} is not in the environment", thing.name());
          false
        }
      }
  }

  pub fn memory(&self) -> &Memory {
    &self.memory
  }

  pub fn opinion_index(&self, opinion: &str) -> Option<usize> {
    self.opinions.iter().position(|o| o.key == opinion)
  }

  pub fn set_opinion(&mut self, index: usize, value: bool) -> Result<(), OpinionError> {
    let opinion = self.opinions.get_mut(index).ok_or(OpinionError::NotFound)?;

    if opinion.key == "you are here" && !value {
      Err(OpinionError::IllegalArgument)
    } else {
      opinion.value = value;
      Ok(())
    }
  }

  pub fn set_execution(&mut self, execution: Execution) {
    self.execution = execution;
  }

  pub fn escape(&self, from: impl Display) {
    println!("{:?} escaped from {from}", self.name);
  }

  pub fn learn_topic(&self, topic: &str) {
    println!("{:?} learned {topic}", self.name);
  }
  pub fn take_exam_topic(&self, topic: &str) {
    println!("{:?} took the {topic} exam", self.name);
  }
  pub fn get_algebraic_expression(&self, _: &str) {}
}

impl ToExecution for Lovable {
  fn to_execution(&self) -> Execution {
    self.execution
  }
}

impl PartialEq for Lovable {
  fn eq(&self, other: &Self) -> bool {
    self.id() == other.id()
  }
}
