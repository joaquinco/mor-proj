use std::cmp;

use rand;
use rand::seq::SliceRandom;

use crate::types::{Cost, Solution, RouteEntry};

/// Assumes the list is sorted
/// Returns an entry of the list from the first 0..size elements
#[allow(dead_code)]
pub fn sized_rcl_choose<'a, T>(list: &'a Vec<T>, size: usize) -> Option<&'a T> {
  let mut rcl: Vec<&T> = vec![];

  for index in 0..cmp::min(size, list.len()) {
    rcl.push(&list[index]);
  }

  match rcl.choose(&mut rand::thread_rng()) {
    None => None,
    Some(&value) => Some(value),
  }
}


// Return the index of the maximun item that satisfies that:
// cost item belongs to [c_min, c_min + (c_max - c_min) * alpha) 
pub fn alpha_max_index(costs: &Vec<f64>, alpha: f64) -> Option<usize> {
  let c_min: f64;

  match costs.first() {
    Some(first_cost) => {
      c_min = *first_cost;
    },
    None => return None,
  }

  let c_max = costs.last().unwrap();
  let max_cost = c_min + (c_max - c_min) * alpha;

  let mut max_index = costs.len();

  match costs.binary_search_by(|cost| cost.partial_cmp(&max_cost).unwrap()) {
    Ok(match_index) => {
      for index in match_index..costs.len() {
        max_index = index;
        if costs[index] > max_cost {
          break;
        }
      }
    },
    Err(index) => {
      max_index = index;
    }
  }

  Some(max_index)
}

/// Assumes the list is sorted
/// Returns an entry of the list assuming the entries that satisfy:
/// c_min <= cost <= c_min + (c_max - c_min) * alpha
#[allow(dead_code)]
pub fn alpha_rcl_choose<'a, T>(
  list: &'a Vec<T>, costs: &Vec<f64>, alpha: f64, min_size: usize,
) -> Option<&'a T> {
  if let Some(max_index) = alpha_max_index(&costs, alpha) {
    sized_rcl_choose(list, cmp::min(cmp::max(max_index, min_size), list.len()))
  } else {
    None
  }
}

/// Returns an entry of the list from the list elements given the probability
/// specify by the weights list.
#[allow(dead_code)]
pub fn weighted_choose<'a, T>(list: &'a Vec<T>, weights: Vec<f64>) -> Option<&'a T> {
  let rcl: Vec<(usize, &T)> = list.iter().enumerate().collect();

  match rcl.choose_weighted(&mut rand::thread_rng(), |(index, _)| weights[*index]) {
    Err(_) => None,
    Ok((_, value)) => Some(value),
  }
}

///
/// Creates a new solution by replacing the two routes
pub fn transform_solution(sol: &Solution, new_route1: &RouteEntry, new_route2: &RouteEntry) -> Solution {
  let mut new_sol = sol.clone();
  let mut new_routes = vec![];

  for route in new_sol.routes {
    new_routes.push({
      if route.vehicle_id == new_route1.vehicle_id {
        new_route1.clone()
      } else if route.vehicle_id == new_route2.vehicle_id {
        new_route2.clone()
      } else {
        route
      }
    })
  }
  new_sol.routes = new_routes.into_iter().filter(|r| r.route_cost() > 0 as Cost).collect();

  new_sol
}
