use crate::config::Config;
use crate::types::Solution;


pub fn run(config: Config) -> Solution {
  info!("Using configuration:\n{}", config);

  config.instance.validate().unwrap();

  let mut iteration = 1;

  while iteration <= config.iters {

    iteration += 1;
  }

  Default::default()
}
