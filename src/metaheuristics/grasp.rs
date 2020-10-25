use std::{cmp, collections::{HashSet, HashMap}};

use rand;
use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};

use crate::types::{Solution, ProblemInstance, RouteEntry, Time, Cost};

#[serde(default)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraspConfig {
  time_weight: f64,
  distance_weight: f64,
  wait_time_enabled: bool,
  rcl_size: usize,
  moves_per_vehicle: usize,
}

impl Default for GraspConfig {
  fn default() -> GraspConfig {
    GraspConfig {
      time_weight: 0.3,
      distance_weight: 0.3,
      wait_time_enabled: false,
      rcl_size: 5,
      moves_per_vehicle: 1,
    }
  }
}

#[derive(Debug)]
struct GraspRouteMove {
  pub vehicle_id: usize,
  pub target_client_id: usize,
  pub cost: f64,
}

#[derive(Debug, Clone)]
struct BasicMove(usize, f64);

#[derive(Default, Debug)]
struct GraspRoute {
  pub vehicle_id: usize,
  pub current_client_id: usize,
  pub current_time: Time,
  pub route_time: Time,
  pub capacity_left: f64,
  pub route: Vec<usize>,
}

impl GraspRoute {
  pub fn update(&mut self, target_client_id: usize, problem: &ProblemInstance) {
    let from_id = self.current_client_id;
    let client_to = &problem.clients[target_client_id];
    let arc_time = problem.distances[from_id][target_client_id];

    /* Update route costs */
    self.current_client_id = target_client_id;
    self.route.push(target_client_id);
    self.capacity_left -= client_to.demand;
    self.route_time += arc_time;

    /* wait time is (if applies): client_to.earliest - current_time - arc_time */
    if from_id == problem.source {
      self.current_time = cmp::max(arc_time, client_to.earliest);
    } else {
      self.current_time += arc_time;
    }
    self.current_time += client_to.service_time;
  }
}

pub struct Grasp {
  pub config: GraspConfig,
}

impl Grasp {
  pub fn iterate(&self, problem: &ProblemInstance) -> Result<Solution, String> {
    match self.build_solution(problem) {
      Err(err) => Err(err),
      Ok(initial_sol) => self.local_search(initial_sol, problem),
    }
  }

  fn local_search(&self, sol: Solution, _problem: &ProblemInstance) -> Result<Solution, String> {
    Ok(sol)
  }

  fn build_solution(&self, problem: &ProblemInstance) -> Result<Solution, String> {
    let mut vehicle_routes = Self::build_grasp_routes(problem);
    let mut all_clients: HashSet<usize> = (0..problem.clients.len())
      .filter(|index| *index != problem.source)
      .map(|index| index.to_owned())
      .collect();

    while !all_clients.is_empty() {
      let mut moves = self.get_possible_moves(&vehicle_routes, &all_clients, &problem);

      moves.sort_by(|m1, m2| m1.cost.partial_cmp(&m2.cost).unwrap());

      let next_move;
      match self.rcl_choose(&moves) {
        Some(value) => next_move = value,
        None => return Err("Couldn't find a feasible solution".to_string()),
      };

      all_clients.remove(&next_move.target_client_id);

      match vehicle_routes.get_mut(&next_move.vehicle_id) {
        Some(vroute) => {
          vroute.update(next_move.target_client_id , problem);
        },
        None => (),
      };
    }

    let mut sol: Solution = Default::default();

    for vehicle in problem.vehicles.iter() {
      let vroute = vehicle_routes.get_mut(&vehicle.id).unwrap();

      /* problem.source is always added to the route */
      if vroute.route.len() < 2 {
        continue
      }

      vroute.update(problem.source, &problem);

      sol.routes.push(RouteEntry {
        vehicle_id: vroute.vehicle_id,
        clients: vroute.route.clone(),
        route_fixed_cost: vehicle.fixed_cost,
        route_time: vroute.route_time,
        route_variable_cost: vroute.route_time as f64 * vehicle.variable_cost,
      });
    }

    Ok(sol)
  }

  fn build_grasp_routes(problem: &ProblemInstance) -> HashMap<usize, GraspRoute> {
    problem.vehicles.iter().map( |vehicle|
      (vehicle.id, GraspRoute {
        vehicle_id: vehicle.id,
        capacity_left: vehicle.capacity,
        current_time: problem.clients[problem.source].earliest,
        current_client_id: problem.source,
        route: vec![problem.source],
        ..Default::default()
      })
    ).collect()
  }

  fn get_possible_moves(
    &self,
    vehicle_routes: &HashMap<usize, GraspRoute>,
    available_clients: &HashSet<usize>,
    problem: &ProblemInstance
  ) -> Vec<GraspRouteMove> {
    let mut ret: Vec<GraspRouteMove> = vec![];

    for vroute in vehicle_routes.values() {
      let mut move_list: Vec<BasicMove>;

      /* Generate list of possible moves for each vehicle */
      move_list = available_clients
        .iter()
        .filter(|&client_id| {
          let client = &problem.clients[*client_id];
          let enough_capacity = client.demand <= vroute.capacity_left;
          let arrival_time = vroute.current_time + problem.distances[vroute.current_client_id][client.id];
          let enough_time = vroute.current_client_id == problem.source || (
            (self.config.wait_time_enabled || client.earliest <= arrival_time) && arrival_time <= client.latest
          );
    
          enough_capacity && enough_time
        })
        .map(|client_id| BasicMove(*client_id, self.compute_move_weight(vroute, *client_id, problem)))
        .collect();

      /* Select best move and add it to moves rcl */
      move_list.sort_by(|BasicMove(_, c1), BasicMove(_, c2)| c1.partial_cmp(c2).unwrap());

      for index in 0..cmp::min(self.config.moves_per_vehicle, move_list.len()) {
        let BasicMove(client_id, cost) = move_list[index];
        ret.push(GraspRouteMove {
          cost: cost,
          target_client_id: client_id,
          vehicle_id: vroute.vehicle_id,
        })
      }
    }

    ret
  }

   ///
   /// Computes the cost of the move: vroute.current_client -> to considering current time
   /// Assumes to client_to satisfies the restrictions of being eligible.
  fn compute_move_weight(&self, vroute: &GraspRoute, client_to: usize, problem: &ProblemInstance) -> f64 {
    let fixed_cost = if problem.source == vroute.current_client_id {
                      problem.vehicles[vroute.vehicle_id].fixed_cost
                    } else {
                      0 as Cost
                    };
    let distance = problem.distances[vroute.current_client_id][client_to];
    let time = (problem.clients[client_to].latest - vroute.current_time - distance) as f64;

    fixed_cost + self.config.distance_weight * distance as f64 + self.config.time_weight * time
  }

  fn rcl_choose<'a, T: std::fmt::Debug>(&self, list: &'a Vec<T>) -> Option<&'a T> {
    let mut rcl: Vec<&T> = vec![];

    for index in 0..cmp::min(self.config.rcl_size, list.len()) {
      rcl.push(&list[index]);
    }

    match rcl.choose(&mut rand::thread_rng()) {
      None => None,
      Some(&value) => {
        Some(value)
      }
    }
  }
}
