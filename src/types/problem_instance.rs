use serde::{Serialize, Deserialize};
use super::{Node, Vehicle, VehicleDefinition, Client, Solution};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProblemInstance {
  #[serde(default)]
  pub source: Node,
  #[serde(default)]
  pub deviation_penalty: f64,
  #[serde(default)]
  pub allowed_deviation: f64,
  pub nodes: Vec<Node>,
  pub distances:  Vec<Vec<f64>>,
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
      nodes: vec![],
      distances: vec![],
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

    self.inited = true;
  }

  fn init_vehicles(&mut self) {
    let mut max: i32 = 1000;

    let vehicles: Vec<Vehicle> = self.vehicle_definitions.iter().flat_map(|vehicle_def| {
      let min = max;
      max = max + vehicle_def.count;
      (min..max).map(move |id| {
        Vehicle { id: id, capacity: vehicle_def.capacity }
      })
    }).collect();

    self.vehicles = vehicles;
  }

  pub fn validate(&self) -> Result<(), String> {
    let node_count = self.nodes.len();

    for (index, distances) in self.distances.iter().enumerate() {
      if distances.len() != node_count {
        return Err(format!("Expected distance vector of {} on index {}", node_count, index));
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
    sol.value = sol.routes.iter().map(|route| route.cost).sum();
  }
}
