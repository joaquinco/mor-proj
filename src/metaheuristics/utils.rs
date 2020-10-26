use std::cmp;

use rand;
use rand::seq::SliceRandom;

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

/// Assumes the list is sorted
/// Returns an entry of the list assuming the entries that satisfy:
/// c_min <= cost <= c_min + (c_max - c_min) * alpha
#[allow(dead_code)]
pub fn alpha_rcl_choose<'a, T>(list: &'a Vec<T>, costs: &Vec<f64>, alpha: f64) -> Option<&'a T> {
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

  sized_rcl_choose(list, max_index + 1)
}

/// Assumes the list is sorted
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
