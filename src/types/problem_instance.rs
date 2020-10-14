use serde::{Serialize, Deserialize};
use super::others::{Node, Truck, TruckDefinition, Client};

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
  pub truck_definitions: Vec<TruckDefinition>,
  #[serde(skip)]
  pub trucks: Option<Vec<Truck>>,
  pub clients: Vec<Client>,
}

impl Default for ProblemInstance {
  fn default() -> ProblemInstance {
    ProblemInstance {
      source: 0,
      deviation_penalty: 0.0,
      allowed_deviation: 0.0,
      nodes: vec![],
      distances: vec![],
      truck_definitions: vec![],
      trucks: None,
      clients: vec![]
    }
  }
}

impl ProblemInstance {
  pub fn validate(&self) -> Result<(), String> {
    let node_count = self.nodes.len();

    for (index, distances) in self.distances.iter().enumerate() {
      if distances.len() != node_count {
        return Err(format!("Expected distance vector of {} on index {}", node_count, index));
      }
    }

    Ok(())
  }

  pub fn trucks(&mut self) -> &Vec<Truck> {
    if self.trucks.is_none() {
      let mut max: i32 = 1000;

      let trucks: Vec<Truck> = self.truck_definitions.iter().flat_map(|truck_def| {
        let min = max;
        max = max + truck_def.count;
        (min..max).map(move |id| {
          Truck { id: id, capacity: truck_def.capacity }
        })
      }).collect();

      self.trucks = Some(trucks);
    }

    self.trucks.as_ref().unwrap()
  }
}
