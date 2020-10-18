use std::fmt;
use serde::{Serialize, Deserialize};

use crate::metaheuristics::GraspConfig;

use super::{ProblemInstance, Solution};

#[serde(default)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub iters: i32,
  pub report_every: i32,
  pub max_error_count: i32,
  pub instance_name: String,
  pub instance: ProblemInstance,
  pub grasp_config: GraspConfig,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      iters: 10,
      report_every: 50,
      max_error_count: 300,
      instance_name: String::from("Some instance"),
      instance: Default::default(),
      grasp_config: Default::default(),
    }
  }
}

impl fmt::Display for Config {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "- instance name: {}\n\
      - iters: {}\n\
      - node count: {}\
      ",
      self.instance_name,
      self.iters,
      self.instance.clients.len()
    )
  }
}

#[derive(Debug, Serialize)]
pub struct Output {
  pub name: String,
  pub solution: Solution,
  pub instance: ProblemInstance,
}
