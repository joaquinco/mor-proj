use serde::{Serialize, Deserialize};

pub type Node = i32;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Vehicle {
  pub id: i32,
  pub capacity: f64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RouteEntry {
  pub vehicle: Node,
  pub nodes: Vec<Vehicle>,
  pub cost: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Client {
  pub id: i32,
  pub demand: f64,
  pub service_time: f64,
  pub earliest: i32,
  pub latest: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VehicleDefinition {
  pub count: i32,
  pub capacity: f64,
}
