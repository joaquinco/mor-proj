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
  pub number_of_threads: i32,
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
      number_of_threads: 1,
    }
  }
}

impl fmt::Display for Config {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "- instance name: {}\n\
      - iters: {}\n\
      - number of threads: {}\n\
      - instance:\n{}",
      self.instance_name,
      self.iters,
      self.number_of_threads,
      self.instance,
    )
  }
}

#[derive(Debug, Serialize)]
pub struct Output {
  pub name: String,
  pub solution: Solution,
  pub instance: ProblemInstance,
}
