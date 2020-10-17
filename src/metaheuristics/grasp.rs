use std::cmp;

use rand;
use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};

use crate::types::{Solution, ProblemInstance, Vehicle, RouteEntry};

#[derive(Debug, Serialize, Deserialize)]
pub struct GraspConfig {
  pub time_weight: f64,
  demand_weight: f64,
  prioritize_larger_vehicles: bool,
  rcl_size: usize,
}

impl Default for GraspConfig {
  fn default() -> GraspConfig {
    GraspConfig {
      time_weight: 0.5,
      demand_weight: 0.5,
      prioritize_larger_vehicles: false,
      rcl_size: 5,
    }
  }
}

pub struct Grasp {
  pub config: GraspConfig,
}

fn remove_from_list(list: &mut Vec<usize>, elem: &usize) {
  match list.binary_search(elem) {
    Ok(index) => { list.remove(index); },
    _ => {}
  };
}

impl Grasp {
  fn build_solution(&self, problem: &ProblemInstance) -> Result<Solution, String> {
    let mut vehicles = self.get_sorted_vehicles(&problem.vehicles);
    let mut all_clients: Vec<usize> = (0..problem.clients.len()).collect();
    all_clients = all_clients
      .iter()
      .filter(|index| **index != problem.source)
      .map(|index| index.to_owned())
      .collect();

    let mut sol: Solution = Default::default();

    loop {
      let selected_vehicle_id;

      match self.rcl_choose(&vehicles) {
        None => return Err("No vehicle left".to_string()),
        Some(value) => selected_vehicle_id = value.to_owned(),
      };

      remove_from_list(&mut vehicles, &selected_vehicle_id);
      let selected_vehicle = &problem.vehicles[selected_vehicle_id];

      let mut capacity_left = selected_vehicle.capacity;
      let mut route_distance = 0f64;
      let mut route: Vec<usize> = vec![];
      let mut current_node = problem.source;

      while capacity_left > 0.0 && !all_clients.is_empty()  {
        let current_clients: Vec<usize> = self.get_sorted_clients(
          capacity_left,
          current_node,
          &all_clients,
          problem
        );

        let selected_client_id;
        
        match self.rcl_choose(&current_clients) {
          None => return Err("No clients could be chosen".to_string()),
          Some(value) => selected_client_id = value.to_owned(),
        };
        remove_from_list(&mut all_clients, &selected_client_id);

        let selected_client = &problem.clients[selected_client_id];

        capacity_left -= selected_client.demand;
        route_distance += problem.distances[current_node][selected_client_id];
        current_node = selected_client_id;

        route.push(selected_client_id);
      }

      if !route.is_empty() {
        route.push(problem.source.to_owned());

        sol.routes.push(RouteEntry {
          vehicle_id: selected_vehicle_id,
          clients: route,
          route_distance: route_distance,
        });
      }

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

  fn get_sorted_clients(&self, capacity: f64, from: usize, available_clients: &Vec<usize>, problem: &ProblemInstance) -> Vec<usize> {
    let mut ret = available_clients.to_vec();
    let client_keys: Vec<f64> = problem.clients
      .iter()
      .map(|client| client.id.to_owned())
      .filter(|id| problem.clients[*id].demand < capacity)
      .map(|id| {
        let index = id.to_owned();
        self.config.demand_weight * problem.clients[index].demand +
        self.config.time_weight * problem.distances[from][index]
      }).collect();

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
}
