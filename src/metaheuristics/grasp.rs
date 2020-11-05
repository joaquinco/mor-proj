use std::{cmp, collections::{HashSet, HashMap}};

use serde::{Serialize, Deserialize};

use crate::types::{Solution, ProblemInstance, RouteEntry, Time, Cost};
use super::utils::{
  alpha_rcl_choose,
  time_max
};

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
}

impl Default for GraspConfig {
  fn default() -> GraspConfig {
    GraspConfig {
      time_weight: 0.3,
      distance_weight: 0.3,
      wait_time_weight: 1.0,
      rcl_size: 5,
      rcl_alpha: 0.3,
      rcl_min_size: 2,
      moves_per_vehicle: 1,
      max_wait_time: 10000 as Time,
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
    self.current_time = time_max(self.current_time + arc_time, client_to.earliest);
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

        let max_overtime = ((client.latest - client.earliest) as f64 * problem.allowed_deviation) as Time;
        let enough_time = arrival_time <= client.latest + max_overtime;

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
