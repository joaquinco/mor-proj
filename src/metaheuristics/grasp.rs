use std::{fmt, cmp, collections::{HashSet, HashMap}};
use std::iter::Iterator;

use serde::{Serialize, Deserialize};
use serde_json;

use crate::types::{
  Cost,
  ProblemInstance,
  RouteEntry,
  RouteEntryClient,
  Solution,
  Time,
};
use crate::utils::time_max;
use super::utils::{alpha_rcl_choose, transform_solution};
use super::local_search::{LocalSearch, LocalSearchNotFound};
use super::insertion_search::insertion_search;
use super::opt2_search::opt2_search;

#[serde(default)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraspConfig {
  time_weight: f64,
  distance_weight: f64,
  wait_time_weight: f64,
  rcl_size: usize,
  rcl_alpha: f64,
  rcl_min_size: usize,
  moves_per_vehicle: usize,
  max_wait_time: Time,
  local_search_iters: i32,
  local_search_first_improvement: bool,
  opt2_search_enabled: bool,
  opt2_search_first_improvement: bool,
  insertion_search_enabled: bool,
  insertion_search_first_improvement: bool,
  insertion_search_sequence_length: usize,
}

impl Default for GraspConfig {
  fn default() -> GraspConfig {
    GraspConfig {
      time_weight: 0.3,
      distance_weight: 0.3,
      wait_time_weight: 1.0,
      rcl_size: 5,
      rcl_alpha: 0.3,
      rcl_min_size: 1,
      moves_per_vehicle: 1,
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

  fn insertion_local_search(&self, sol: &Solution, problem: &ProblemInstance) -> Option<Solution> {
    let ls = LocalSearch::new(self.config.local_search_first_improvement);

    ls.iterate(&sol.routes, &sol.routes, |_index1, route1, _index2, route2| {
      if route1.vehicle_id == route2.vehicle_id {
        return Err(LocalSearchNotFound);
      }

      let local_search_result = insertion_search(
        problem,
        route1,
        route2,
        self.config.insertion_search_sequence_length,
        self.config.insertion_search_first_improvement
      );

      if let Some((new_route1, new_route2)) = local_search_result {
        let mut new_sol = transform_solution(sol, &new_route1, &new_route2);
        problem.evaluate_sol(&mut new_sol);
        let ret_val = new_sol.value;

        Ok((new_sol, ret_val))
      } else {
        Err(LocalSearchNotFound)
      }
    })
  }

  fn opt2_local_search(&self, sol: &Solution, problem: &ProblemInstance) -> Option<Solution> {
    let ls = LocalSearch::new(self.config.local_search_first_improvement);

    ls.iterate(&sol.routes, &sol.routes, |_index1, route1, _index2, route2| {
      if route1.vehicle_id == route2.vehicle_id {
        return Err(LocalSearchNotFound)
      }

      let local_search_result = opt2_search(
        problem, route1, route2, self.config.opt2_search_first_improvement
      );
      if let Some((new_route1, new_route2)) = local_search_result {
        let mut new_sol = transform_solution(sol, &new_route1, &new_route2);
        problem.evaluate_sol(&mut new_sol);
        let value = new_sol.value;
  
        Ok((new_sol, value))
      } else {
        Err(LocalSearchNotFound)
      }
    })
  }

  fn local_search(&self, sol: Solution, problem: &ProblemInstance) -> Result<Solution, String> {
    let mut best_sol = sol;
    let mut iteration = self.config.local_search_iters;
    let mut should_break = false;

    while iteration > 0 {
      iteration -= 1;

      if self.config.opt2_search_enabled {
        if let Some(new_sol) = self.opt2_local_search(&best_sol, problem) {
          best_sol = new_sol;
        } else {
          should_break = true;
        }
      }

      if self.config.insertion_search_enabled {
        if let Some(new_sol) = self.insertion_local_search(&best_sol, problem) {
          best_sol = new_sol
        } else {
          should_break = true;
        }
      }

      if should_break {
        break
      }
    }

    Ok(best_sol)
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
        demand: vehicle.capacity - vroute.capacity_left,
      });
    }

    problem.evaluate_sol(&mut sol);
    Ok(sol)
  }

  fn build_grasp_routes(problem: &ProblemInstance) -> HashMap<usize, GraspRoute> {
    problem.vehicles.iter().map(|vehicle| {
      let mut grasp_route = GraspRoute {
        vehicle_id: vehicle.id,
        capacity_left: vehicle.capacity,
        current_time: problem.clients[problem.source].earliest,
        current_client_id: problem.source,
        ..Default::default()
      };

      grasp_route.update(problem.source, problem);

      (vehicle.id, grasp_route)
    }).collect()
  }

  fn get_possible_moves(
    &self,
    vehicle_routes: &HashMap<usize, GraspRoute>,
    available_clients: &HashSet<usize>,
    problem: &ProblemInstance
  ) -> Vec<GraspRouteMove> {
    let mut ret: Vec<GraspRouteMove> = vec![];

    for vroute in vehicle_routes.values() {
      /* Generate list of possible moves for each vehicle */
      let mut move_list = vec![];
      for client_id in available_clients {
        let client = &problem.clients[*client_id];
        let enough_capacity = client.demand <= vroute.capacity_left;
        let mut arrival_time = vroute.current_time + problem.distances[vroute.current_client_id][client.id];

        /* If current_time + distance is less than client.earliest, the vehicle can wait */
        let wait_time = time_max(client.earliest - arrival_time, 0 as Time);

        if wait_time > self.config.max_wait_time {
          continue
        }

        arrival_time = time_max(arrival_time, client.earliest);

        let enough_time = problem.is_move_feasible(
          vroute.current_client_id,
          client.id,
          vroute.current_time,
        );

        if !(enough_capacity && enough_time) {
          continue
        }

        let move_cost = self.compute_move_cost(problem, vroute, *client_id, arrival_time, wait_time);
        move_list.push(BasicMove(*client_id, move_cost));
      }

      /* Sort moves by cost and select the <moves_per_vehicle> bests */
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
   /// Computes the cost of the move: from vroute.current_client to client_to considering arrival time
   /// Assumes to client_to satisfies the restrictions of being eligible.
  fn compute_move_cost(
    &self,
    problem: &ProblemInstance,
    vroute: &GraspRoute,
    client_to: usize,
    arrival_time: Time,
    wait_time: Time
  ) -> f64 {
    let fixed_cost = if problem.source == vroute.current_client_id {
                      20.0 * problem.vehicles[vroute.vehicle_id].fixed_cost
                    } else {
                      0 as Cost
                    };
    let distance = problem.distances[vroute.current_client_id][client_to];
    let client = &problem.clients[client_to];
    let close_proximity_time: Time = time_max(client.latest - arrival_time, 0 as Time);
    let overtime = time_max(arrival_time - client.latest, 0 as Time);

    fixed_cost
    + self.config.distance_weight * distance as f64
    + self.config.time_weight * close_proximity_time as f64
    + self.config.wait_time_weight * wait_time as f64
    + problem.deviation_penalty * overtime as f64
  }

  fn rcl_choose<'a>(&self, moves: &'a Vec<GraspRouteMove>) -> Option<&'a GraspRouteMove> {
    let costs: Vec<f64> = moves.iter().map(|m| m.cost).collect();

    alpha_rcl_choose(moves, &costs, self.config.rcl_alpha, self.config.rcl_min_size)
  }
}
