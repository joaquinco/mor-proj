use std::fmt;
use serde::{Serialize, Deserialize};

use super::problem_instance::ProblemInstance;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  #[serde(default)]
  pub iters: i32,
  #[serde(default)]
  pub instance_name: String,
  pub instance: ProblemInstance,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      iters: 10,
      instance_name: String::from("Some instance"),
      instance: Default::default(),
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
      self.instance.nodes.len()
    )
  }
}
