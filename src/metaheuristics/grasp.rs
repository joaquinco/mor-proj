use crate::types::{Solution, Config};

pub struct Grasp {
  pub time_weight: f64,
  pub demand_weight: f64,
  pub max_iter: i32,
}

impl Default for Grasp {
  fn default() -> Grasp {
    Grasp {
      time_weight: 0.5,
      demand_weight: 0.5,
      max_iter: 20,
    }
  }
}

impl Grasp {
  pub fn init(&self, conf: Config) {

  }

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
