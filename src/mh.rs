use crate::types::{Config, Solution};

pub fn run(config: Config) -> Solution {
  info!("Using configuration:\n{}", config);

  let mut iteration = 1;

  while iteration <= config.iters {

    iteration += 1;
  }

  Default::default()
}
