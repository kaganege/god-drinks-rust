use std::fmt::Debug;

pub struct Attribute {
  name: String,
  value: Option<String>,
}

impl Attribute {
  pub fn new(name: impl AsRef<str>) -> Self {
    Self {
      name: name.as_ref().to_string(),
      value: None,
    }
  }

  pub fn new_with_value(name: impl AsRef<str>, value: impl AsRef<str>) -> Self {
    Self {
      name: name.as_ref().to_string(),
      value: Some(value.as_ref().to_string()),
    }
  }
}
pub trait ToAttribute {
  fn to_attribute(&self) -> Attribute;
}

impl<T> ToAttribute for &[T]
where
  T: ToAttribute + Debug,
{
  fn to_attribute(&self) -> Attribute {
    if let Some(first) = self.first() {
      Attribute::new_with_value(first.to_attribute().name, format!("{:?}", self))
    } else {
      Attribute::new("unknown")
    }
  }
}
