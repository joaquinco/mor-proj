use crate::config::Config;
use crate::types::Solution;

pub fn run(config: Config) -> Solution {

  let mut iteration = 1;

  while iteration <= config.iters {
    debug!("Iteration {}", iteration);

    iteration += 1;
  }

  Default::default()
}
