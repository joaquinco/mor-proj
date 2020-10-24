use std::fmt;
use serde::{Serialize, Deserialize};

use super::others::{RouteEntry, Cost, Time};

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
  - distance: {}
  - routes:\n{}",
      self.value,
      self.total_route_time(),
      self.routes.iter().map(|route| format!("{}", route)).collect::<Vec<String>>().join("\n")
    )
  }
}

impl Solution {
  pub fn total_route_time(&self) -> Time {
    self.routes.iter().map(|route| route.route_time).sum()
  }
}
