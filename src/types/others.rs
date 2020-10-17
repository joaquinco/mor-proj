use serde::{Serialize, Deserialize};

type Time = i32;

#[derive(Debug, Default, Serialize)]
pub struct Vehicle {
  /* id is the index */
  pub id: usize,
  pub capacity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleDefinition {
  pub count: i32,
  pub capacity: f64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RouteEntry {
  pub vehicle_id: usize,
  pub clients: Vec<usize>,
  pub route_distance: f64,
  pub route_time: Time,
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
}
