use crate::types::{Config, Solution};
use crate::metaheuristics::Grasp;

pub fn run(config: Config) -> Solution {
  info!("Using configuration:\n{}", config);

  let mut iteration = config.iters;
  let mut best: Option<Solution> = None;
  let mh: Grasp = Grasp { config: config.grasp_config };

  while iteration != 0 {
    let mut sol = match mh.iterate(&config.instance) {
      Err(error) => panic!("Metaheuristic returned no solution: {}", error),
      Ok(value) => value,
    };

    config.instance.evaluate_sol(&mut sol);

    match best.as_ref() {
      None => best = Some(sol),
      Some(current) => {
        if current.value > sol.value {
          best = Some(sol)
        }
      }
    }

    if iteration % 10 == 0 {
      let best_value = best.as_ref().unwrap().value;
      debug!("Iteration #{}, best value: {}", iteration, best_value);
    }

    iteration -= 1;
  }

  best.unwrap()
}
