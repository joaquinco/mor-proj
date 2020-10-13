use serde::{Serialize, Deserialize};

pub type Node = i32;
pub type Truck = i32;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RouteEntry {
  pub vehicle: Node,
  pub nodes: Vec<Truck>,
  pub cost: f64,
}
