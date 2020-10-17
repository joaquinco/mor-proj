use serde::{Serialize, Deserialize};
use super::{Vehicle, VehicleDefinition, Client, Solution};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProblemInstance {
  #[serde(default)]
  pub source: usize,
  #[serde(default)]
  pub deviation_penalty: f64,
  #[serde(default)]
  pub allowed_deviation: f64,
  pub distances:  Vec<Vec<f64>>,
  pub times: Vec<Vec<i32>>,
  pub vehicle_definitions: Vec<VehicleDefinition>,
  #[serde(skip)]
  pub vehicles: Vec<Vehicle>,
  pub clients: Vec<Client>,
  #[serde(skip)]
  inited: bool,
}

impl Default for ProblemInstance {
  fn default() -> ProblemInstance {
    ProblemInstance {
      source: 0,
      deviation_penalty: 0.0,
      allowed_deviation: 0.0,
      distances: vec![],
      times: vec![],
      vehicle_definitions: vec![],
      vehicles: vec![],
      clients: vec![],
      inited: false,
    }
  }
}

impl ProblemInstance {
  pub fn init(&mut self) {
    if self.inited {
      return
    }

    self.init_vehicles();
    self.init_clients();

    self.inited = true;
  }

  fn init_vehicles(&mut self) {
    let mut max: usize = 0;

    let vehicles: Vec<Vehicle> = self.vehicle_definitions.iter().flat_map(|vehicle_def| {
      let min = max;
      max = max + vehicle_def.count as usize;
      (min..max).map(move |id| {
        Vehicle { id: id, capacity: vehicle_def.capacity }
      })
    }).collect();

    self.vehicles = vehicles;
  }

  fn init_clients(&mut self) {
    for index in 0..self.clients.len() {
      self.clients[index].id = index;
    }
  }

  pub fn validate(&self) -> Result<(), String> {
    let node_count = self.clients.len();

    for (index, distances) in self.distances.iter().enumerate() {
      if distances.len() != node_count {
        return Err(format!("Expected distance vector of {} on index {}", node_count, index));
      }
    }

    for (index, distances) in self.times.iter().enumerate() {
      if distances.len() != node_count {
        return Err(format!("Expected time vector of {} on index {}", node_count, index));
      }
    }

    if self.vehicles.len() == 0 {
      return Err("You must specify at least one vehicle".to_string());
    }

    if self.clients.len() == 0 {
      return Err("You must specify some clients".to_string());
    }

    Ok(())
  }
  
  pub fn evaluate_sol(&self, sol: &mut Solution) {
    sol.value = sol.routes.iter().map(|route| route.route_distance).sum();
  }
}
