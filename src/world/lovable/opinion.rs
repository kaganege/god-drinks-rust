#[derive(Clone, PartialEq, Eq)]
pub struct Opinion<K, V> {
  pub key: K,
  pub value: V,
}

impl<K, V> Opinion<K, V> {
  pub fn new<Q, R>(key: Q, value: R) -> Self
  where
    Q: Into<K>,
    R: Into<V>,
  {
    Self {
      key: key.into(),
      value: value.into(),
    }
  }

  pub fn with_key<Q>(key: Q) -> Self
  where
    Q: Into<K>,
    V: Default,
  {
    Self {
      key: key.into(),
      value: Default::default(),
    }
  }
}
