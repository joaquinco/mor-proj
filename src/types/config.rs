use std::fmt;
use serde::{Serialize, Deserialize};
use serde_json;

use crate::metaheuristics::GraspConfig;

use super::{ProblemInstance, Solution};

#[serde(default)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub iters: i32,
  pub grasp_config: GraspConfig,
  pub number_of_threads: i32,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      iters: 10,
      grasp_config: Default::default(),
      number_of_threads: 1,
    }
  }
}

impl fmt::Display for Config {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
  }
}

#[derive(Debug, Serialize)]
pub struct Output {
  pub name: String,
  pub solution: Solution,
  pub instance: ProblemInstance,
}
