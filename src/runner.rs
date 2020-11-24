use crossbeam;

use crate::types::{Config, Solution};
use crate::metaheuristics::Grasp;

fn do_run(thread_id: i32, config: &Config) -> Option<Solution> {
  let mut iteration = config.iters;
  let mut best: Option<Solution> = None;
  let mh: Grasp = Grasp { config: config.grasp_config.clone() };
  let mut error_count = 0;
  let mut last_error: String = "".to_string();

  while iteration != 0 {
    iteration -= 1;

    let sol = match mh.iterate(&config.instance) {
      Err(error) => {
        last_error = error;
        error_count += 1;
        continue;
      },
      Ok(mut value) => {
        value.iter_found = iteration;
        value
      },
    };


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
      info!("Thread #{} - iteration #{}, best value: {}", thread_id, config.iters - iteration, best_value);
    }
  }

  if error_count > 0 {
    error!("Thread #{} - solution not found on {} iterations", thread_id, error_count);
    error!("Thread #{} - last error was: {}", thread_id, last_error);
  }

  best
}

pub fn run(config: &Config) -> Option<Solution> {
  info!("Using configuration:\n{}", config);

  let mut results = vec![];

  crossbeam::scope(|s| {
    let mut handlers = vec![];

    for index in 0..config.number_of_threads {
      let handle = s.spawn(move |_| do_run(index + 1, config));
      handlers.push(handle);
    }

    for handle in handlers {
      match handle.join() {
        Ok(result) => results.push(result),
        Err(_) => error!("Error joining thread"),
      }
    }
  }).unwrap();

  if results.is_empty() {
    return None;
  }

  let mut ret = results.pop().unwrap();
  while !results.is_empty() {
    let current = results.pop().unwrap();

    if ret.is_none() {
      ret = current;
    } else if current.is_some() {
      let s1 = ret.as_ref().unwrap();
      let s2 = current.as_ref().unwrap();

      if s2.value < s1.value {
        ret = current;
      }
    }
  }

  ret
}
