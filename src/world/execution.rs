#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Execution;

pub trait ToExecution {
  fn to_execution(&self) -> Execution;
}
