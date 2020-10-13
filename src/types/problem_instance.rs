use serde::{Serialize, Deserialize};
use super::others::Node;

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
