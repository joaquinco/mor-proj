use crate::types::{Config, Solution};
use crate::metaheuristics::Grasp;

pub fn run(config: &Config) -> Solution {
  info!("Using configuration:\n{}", config);

  let mut iteration = config.iters;
  let mut best: Option<Solution> = None;
  let mh: Grasp = Grasp { config: config.grasp_config.clone() };
  let mut error_count = 0;
  let mut last_error: String = "".to_string();

  while iteration != 0 {
    let mut sol = match mh.iterate(&config.instance) {
      Err(error) => {
        last_error = error;
        error_count += 1;
        continue;
      },
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

    if iteration % config.report_every == 0 {
      let best_value = best.as_ref().unwrap().value;
      debug!("Iteration #{}, best value: {}", config.iters - iteration, best_value);
    }

    iteration -= 1;
  }

  if error_count > 0 {
    error!("Solution not found on {} iterations", error_count);
    error!("Last error was: {}", last_error);
  }

  best.unwrap()
}
