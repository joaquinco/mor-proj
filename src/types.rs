use std::fmt;

#[derive(Debug, Default)]
pub struct Solution {
  pub value: i32
}

impl fmt::Display for Solution {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "<Solution value={}>", self.value)
  }
}
