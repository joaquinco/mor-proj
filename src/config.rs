use std::fmt;
use serde::{Serialize, Deserialize};
use crate::types::Node;

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

#[derive(Debug, Deserialize, Serialize)]
pub struct ProblemInstance {
  #[serde(default)]
  pub source: Node,
  pub nodes: Vec<Node>,
  pub distances:  Vec<Vec<f64>>,
}

impl Default for ProblemInstance {
  fn default() -> ProblemInstance {
    ProblemInstance {
      source: 0,
      nodes: vec![],
      distances: vec![],
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
}
