use serde::{Deserialize, Serialize};

#[serde(default)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
  pub iters: i32,
  pub some_param: String,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      iters: 10,
      some_param: String::from("Something")
    }
  }
}
