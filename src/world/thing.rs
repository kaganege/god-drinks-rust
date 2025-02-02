use std::any::Any;

use super::World;

pub use attribute::{Attribute, ToAttribute};
pub use proof::Proof;

mod attribute;
mod proof;

#[macro_export]
macro_rules! impl_thing {
  ($thing:ty $({$($rest:tt)+})?) => {
    impl $crate::world::Thing for $thing {
      fn as_any(&self) -> &dyn std::any::Any {
        self
      }

      fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
      }

      $(
        $($rest)+
      )?
    }
  };

  ($thing:ty, $id_param:ident) => {
    $crate::impl_thing! {
      $thing {
        fn id(&self) -> u64 {
          self.$id_param
        }
      }
    }
  };
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Current {
  #[default]
  AC,
  DC,
}

impl Current {
  pub fn toggle(&self) -> Self {
    match self {
      Current::AC => Current::DC,
      Current::DC => Current::AC,
    }
  }
}

pub trait Thing {
  fn as_any(&self) -> &dyn Any;
  fn as_mut_any(&mut self) -> &mut dyn Any;

  fn id(&self) -> u64 {
    unimplemented!()
  }

  fn name(&self) -> &str {
    std::any::type_name::<Self>()
  }

  fn add_attribute(&mut self, _: Attribute) {}
  fn add_action(&mut self, _: &'static str, _: &dyn Any) {}

  fn x_position(&self) -> u32 {
    0
  }

  fn to_limit(&self) -> u32 {
    0
  }

  fn get_num_stimulations_available(&self) -> u32 {
    10
  }

  fn get_num_stimulations_needed(&self) -> u32 {
    1
  }

  fn request_execution(&self, _: &World) {}

  fn set_proof(&mut self, proof: Proof) {
    println!("{:?} prove {proof}", self.name());
  }

  fn to_proof(&self) -> Proof {
    Proof(self.name().to_string())
  }
}

// dyn_clone::clone_trait_object!(Thing);
