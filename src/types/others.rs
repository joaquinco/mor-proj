use std::fmt;

use serde::{Serialize, Deserialize};

pub type Time = f64;
pub type Cost = f64;

#[derive(Debug, Default, Serialize)]
pub struct Vehicle {
  /* id is the index */
  pub id: usize,
  pub capacity: f64,
  pub fixed_cost: Cost,
  pub variable_cost: Cost,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleDefinition {
  pub count: i32,
  pub capacity: f64,
  pub fixed_cost: Cost,
  pub variable_cost: Cost,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Client {
  /* id is the index */
  #[serde(skip_deserializing)]
  pub id: usize,
  pub demand: f64,
  pub service_time: Time,
  pub earliest: Time,
  pub latest: Time,
  /* pos is not actually used but needed to draw the result from the output */
  pub pos: [f64; 2],
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct RouteEntryClient {
  pub client_id: usize,
  pub arrive_time: Time,
  pub leave_time: Time,
  pub wait_time: Time,
}

#[derive(Debug, Default, Serialize)]
pub struct RouteEntry {
  pub vehicle_id: usize,
  pub clients: Vec<RouteEntryClient>,
  pub route_time: Time,
  pub route_fixed_cost: Cost,
  pub route_variable_cost: Cost,
}

impl RouteEntry {
  pub fn route_cost(&self) -> Cost {
    self.route_fixed_cost + self.route_variable_cost
  }
}

impl fmt::Display for RouteEntry {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
"    - vehicle_id: {}
      route: {}
      route time: {}
      fixed cost: {}
      variable cost: {}",
      self.vehicle_id,
      self.clients.iter().map(|client| client.client_id.to_string()).collect::<Vec<String>>().join(", "),
      self.route_time,
      self.route_fixed_cost,
      self.route_variable_cost,
    )
  }
}

