use std::fmt;
use serde::{Serialize, Deserialize};

use super::others::RouteEntry;

#[derive(Debug, Serialize, Deserialize)]
pub struct Solution {
  pub routes: Vec<RouteEntry>,
  #[serde(skip_deserializing)]
  pub value: f64,
}

impl Default for Solution {
  fn default() -> Solution {
    Solution {
      routes: vec![],
      value: (1 << 31) as f64,
    }
  }
}

impl fmt::Display for Solution {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "<Solution value={} routes={}>", self.value, self.routes.len())
  }
}
