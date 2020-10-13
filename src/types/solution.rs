use std::fmt;
use serde::{Serialize, Deserialize};

use super::others::RouteEntry;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Solution {
  pub routes: Vec<RouteEntry>,
  #[serde(skip_deserializing)]
  value: Option<f64>,
}

impl fmt::Display for Solution {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let value = self.value.unwrap_or(-1.0);
    write!(f, "<Solution value={} routes={}>", value, self.routes.len())
  }
}

impl Solution {
  pub fn value(&mut self) -> f64 {
    if self.value.is_none() {
      self.value = Some(self.compute_value());
    }
    
    self.value.unwrap()
  }

  fn compute_value(&self) -> f64 {
    self.routes.iter().map(|route| route.cost).sum()
  }
}
