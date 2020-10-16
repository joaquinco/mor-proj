use serde::{Serialize, Deserialize};

use crate::types::{Solution, Config};

#[derive(Debug, Serialize, Deserialize)]
pub struct GraspConfig {
  pub time_weight: f64,
  demand_weight: f64,
  prioritize_larger_vehicles: bool,
}

impl Default for GraspConfig {
  fn default() -> GraspConfig {
    GraspConfig {
      time_weight: 0.5,
      demand_weight: 0.5,
      prioritize_larger_vehicles: false,
    }
  }
}

pub struct Grasp {
  pub config: GraspConfig,
}

impl Grasp {
  fn build_solution(&self) -> Solution {
    Default::default()
  }

  fn local_search(&self, sol: Solution) -> Solution {
    sol
  }

  pub fn iterate(&self) -> Solution {  
    self.local_search(self.build_solution())
  }
}
