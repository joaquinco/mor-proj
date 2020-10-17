use std::fmt;
use serde::{Serialize, Deserialize};

use super::others::{RouteEntry, Cost};

#[derive(Debug, Serialize, Deserialize)]
pub struct Solution {
  pub routes: Vec<RouteEntry>,
  #[serde(skip_deserializing)]
  pub value: Cost,
}

impl Default for Solution {
  fn default() -> Solution {
    Solution {
      routes: vec![],
      value: (1 << 31) as Cost,
    }
  }
}

impl fmt::Display for Solution {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
"Solution:
  - value: {}
  - routes:\n{}",
      self.value,
      self.routes.iter().map(|route| format!("{}", route)).collect::<Vec<String>>().join("\n")
    )
  }
}
