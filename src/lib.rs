extern crate protobuf;

// Protobuf generated files
mod proto_client;

pub trait Versioned {
  fn version(&self) -> i32;
}

pub trait HasMerge {
  fn merge(&self, other: &Self) -> Self;
}

pub struct WithVersion<T> {
  _version: i32,
  pub value: T
}

impl<T: Clone> WithVersion<T> {
  pub fn new(value: &T) -> WithVersion<T> {
    WithVersion::<T> { _version: 0, value: value.clone() }
  }

  pub fn next(&self, value: &T) -> WithVersion<T> {
    WithVersion::<T> { _version: self._version + 1, value: value.clone() }
  }
}

impl<T> Versioned for WithVersion<T> {
  fn version(&self) -> i32 {
    self._version
  }
}

impl<T> HasMerge for T
    where T: Versioned + Clone {
    fn merge(&self, other: &Self) -> Self {
    match self.version() < other.version() {
      true => self.clone(),
      false => other.clone()
    }
  }
}
