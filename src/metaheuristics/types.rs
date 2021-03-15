use std::fmt;

use serde::{Serialize, Deserialize};
use serde_json;

use crate::types::{Time, ProblemInstance, RouteEntryClient};

#[serde(default)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraspConfig {
  pub time_weight: f64,
  pub distance_weight: f64,
  pub wait_time_weight: f64,
  pub rcl_alpha: f64,
  pub rcl_min_size: usize,
  pub moves_per_vehicle_min_size: usize,
  pub moves_per_vehicle_alpha: f64,
  pub max_wait_time: Time,
  pub local_search_iters: i32,
  pub local_search_first_improvement: bool,
  pub opt2_search_enabled: bool,
  pub opt2_search_first_improvement: bool,
  pub insertion_search_enabled: bool,
  pub insertion_search_first_improvement: bool,
  pub insertion_search_sequence_length: usize,
}

impl Default for GraspConfig {
  fn default() -> GraspConfig {
    GraspConfig {
      time_weight: 0.1,
      distance_weight: 0.7,
      wait_time_weight: 0.2,
      rcl_alpha: 0.3,
      rcl_min_size: 1,
      moves_per_vehicle_min_size: 2,
      moves_per_vehicle_alpha: 0.05,
      max_wait_time: 10000 as Time,
      local_search_iters: 100,
      local_search_first_improvement: true,
      opt2_search_enabled: true,
      opt2_search_first_improvement: false,
      insertion_search_enabled: true,
      insertion_search_first_improvement: true,
      insertion_search_sequence_length: 1,
    }
  }
}

impl fmt::Display for GraspConfig {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
  }
}

#[derive(Debug)]
pub struct GraspRouteMove {
  pub vehicle_id: usize,
  pub target_client_id: usize,
  pub cost: f64,
}

#[derive(Default, Debug)]
pub struct GraspRoute {
  pub vehicle_id: usize,
  pub current_client_id: usize,
  pub current_time: Time,
  pub route_time: Time,
  pub capacity_left: f64,
  pub route: Vec<RouteEntryClient>,
}

impl GraspRoute {
  pub fn update(&mut self, target_client_id: usize, problem: &ProblemInstance) {
    let client_to = &problem.clients[target_client_id];
    let arc_time = problem.distances[self.current_client_id][target_client_id];

    /* Update route costs */
    self.current_client_id = target_client_id;
    self.capacity_left -= client_to.demand;
    self.route_time += arc_time;

    let route_entry_client = problem.create_route_entry_client(
      arc_time, target_client_id, self.current_time
    );
    self.current_time = route_entry_client.leave_time;

    self.route.push(route_entry_client);
  }
}
