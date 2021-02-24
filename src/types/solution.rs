use std::fmt;
use serde::{Serialize};

use super::others::{RouteEntry, Cost, Time};

#[derive(Debug, Clone, Serialize)]
pub struct Solution {
  pub routes: Vec<RouteEntry>,
  pub value: Cost,
  pub construction_value: Cost,
  pub distance: Time,
  pub iter_found: i32,
}

impl Default for Solution {
  fn default() -> Solution {
    Solution {
      routes: vec![],
      distance: 0 as Time,
      value: (1 << 31) as Cost,
      construction_value: 0 as Cost,
      iter_found: 0,
    }
  }
}

impl fmt::Display for Solution {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
"Solution:
  value: {}
  construction_value: {}
  distance: {}
  found at iter: {}
  routes:\n{}",
      self.value,
      self.construction_value,
      self.distance,
      self.iter_found,
      self.routes.iter().map(|route| format!("{}", route)).collect::<Vec<String>>().join("\n")
    )
  }
}

impl Solution {
  pub fn total_route_time(&self) -> Time {
    self.routes.iter().map(|route| route.route_time).sum()
  }
}
