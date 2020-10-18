use std::{cmp, collections::HashSet};

use rand;
use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};

use crate::utils::remove_from_vec;
use crate::types::{Solution, ProblemInstance, Vehicle, RouteEntry, Client, Time, Cost};

#[serde(default)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraspConfig {
  time_weight: f64,
  demand_weight: f64,
  distance_weight: f64,
  prioritize_larger_vehicles: bool,
  rcl_size: usize,
}

impl Default for GraspConfig {
  fn default() -> GraspConfig {
    GraspConfig {
      time_weight: 0.3,
      demand_weight: 0.3,
      distance_weight: 0.3,
      prioritize_larger_vehicles: false,
      rcl_size: 5,
    }
  }
}

pub struct Grasp {
  pub config: GraspConfig,
}

impl Grasp {
  fn build_solution(&self, problem: &ProblemInstance) -> Result<Solution, String> {
    let mut vehicles = self.get_sorted_vehicles(&problem.vehicles);
    let mut all_clients: HashSet<usize> = (0..problem.clients.len())
      .filter(|index| *index != problem.source)
      .map(|index| index.to_owned())
      .collect();

    let mut sol: Solution = Default::default();

    loop {
      let selected_vehicle_id;

      match self.rcl_choose(&vehicles) {
        None => return Err("No vehicle left".to_string()),
        Some(value) => selected_vehicle_id = value.to_owned(),
      };

      remove_from_vec(&mut vehicles, &selected_vehicle_id);
      let selected_vehicle = &problem.vehicles[selected_vehicle_id];

      let mut capacity_left = selected_vehicle.capacity;
      let mut route_distance = 0 as Time;
      let mut route: Vec<usize> = vec![problem.source];
      let mut current_node = problem.source;
      let mut current_time = problem.clients[problem.source].earliest;

      while capacity_left > 0.0 && !all_clients.is_empty() {
        let current_clients: Vec<usize> = self.get_sorted_clients(
          capacity_left,
          current_time,
          current_node,
          &all_clients,
          problem
        );

        let selected_client_id;
        
        /* Choose a client */
        match self.rcl_choose(&current_clients) {
          /* No feasible clients to chose */
          None => break,
          Some(value) => selected_client_id = value.to_owned(),
        };
        all_clients.remove(&selected_client_id);
        let selected_client = &problem.clients[selected_client_id];

        /* Update route costs */
        let arc_time = problem.distances[current_node][selected_client_id];
        capacity_left -= selected_client.demand;
        route_distance += arc_time;
        current_time += arc_time + selected_client.service_time;
        route.push(selected_client_id);
        
        current_node = selected_client_id;
      }

      /* Add costs for going back to source */
      route.push(problem.source.to_owned());
      route_distance += problem.distances[current_node][problem.source];
      /* TODO: we assume here that the source.latest time is big enough,
        * so we don't verify if time of arrival is lower than source latest.
        */

      /* Add route to current solution */
      sol.routes.push(RouteEntry {
        vehicle_id: selected_vehicle_id,
        clients: route,
        route_time: route_distance,
        route_fixed_cost: selected_vehicle.fixed_cost,
        route_variable_cost: route_distance as Cost * selected_vehicle.variable_cost,
      });

      if all_clients.is_empty() {
        break;
      }
    }

    Ok(sol)
  }

  fn local_search(&self, sol: Solution, _problem: &ProblemInstance) -> Result<Solution, String> {
    Ok(sol)
  }

  pub fn iterate(&self, problem: &ProblemInstance) -> Result<Solution, String> {
    match self.build_solution(problem) {
      Err(err) => Err(err),
      Ok(initial_sol) => self.local_search(initial_sol, problem),
    }
  }

  fn get_sorted_vehicles(&self, initial_vehicles: &Vec<Vehicle>) -> Vec<usize> {
    let mut vehicles: Vec<usize> = (0..initial_vehicles.len()).collect();

    vehicles.sort_by(|a, b|
      initial_vehicles[a.to_owned()].capacity.partial_cmp(
        &initial_vehicles[b.to_owned()].capacity
      ).unwrap()
    );

    if self.config.prioritize_larger_vehicles {
      vehicles.reverse();
    }

    vehicles
  }

  fn get_sorted_clients(
    &self,
    capacity: f64,
    current_time: i32,
    from: usize,
    available_clients: &HashSet<usize>,
    problem: &ProblemInstance
  ) -> Vec<usize> {
    let mut feasible_clients = HashSet::new();

    /* Consider clients that satisfy the restrictions */
    for client_id in available_clients.iter() {
      let enough_capacity = problem.clients[*client_id].demand < capacity;
      let enough_time = problem.clients[*client_id].latest > current_time + problem.distances[from][*client_id];

      if enough_capacity && enough_time {
        feasible_clients.insert(client_id.to_owned());
      }
    }

    let mut ret: Vec<usize> = feasible_clients.iter().cloned().collect();

    let client_keys: Vec<f64> = problem.clients
      .iter()
      .map(|client| self.compute_client_weight(
        client,
        problem.distances[from][client.id],
        current_time,
      ))
      .collect();

    ret.sort_by(|a, b| {
      client_keys[a.to_owned()].partial_cmp(&client_keys[b.to_owned()]).unwrap()
    });

    ret
  }

  fn rcl_choose(&self, list: &Vec<usize>) -> Option<usize> {
    let rcl = list[0..cmp::min(self.config.rcl_size, list.len())].to_vec();

    match rcl.choose(&mut rand::thread_rng()) {
      None => None,
      Some(value) => Some(value.to_owned())
    }
  }

  /* Client weight depends on:
   * - Distance from current node
   * - Demand
   * - Next to be unavailable
   */
  fn compute_client_weight(&self, client: &Client, distance: Time, current_time: Time) -> f64 {
    self.config.demand_weight * client.demand +
    self.config.distance_weight * distance as f64 +
    self.config.time_weight * (client.earliest - current_time) as f64
  }
}
