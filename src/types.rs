use std::fmt;

use serde::{Serialize, Deserialize};

pub type Node = i32;
pub type Truck = i32;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RouteEntry {
  pub vehicle: Node,
  pub nodes: Vec<Truck>,
  pub cost: f64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Solution {
  pub value: f64,
  pub routes: Vec<RouteEntry>,
}

impl fmt::Display for Solution {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "<Solution value={} routes={}>", self.value, self.routes.len())
  }
}
