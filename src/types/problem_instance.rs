use std::fmt;

use serde::{Serialize, Deserialize};
use crate::utils::time_max;
use super::{Vehicle, VehicleDefinition, Client, Solution, Time, Cost, RouteEntry, RouteEntryClient};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ProblemInstance {
  pub name: String,
  pub source: usize,
  pub deviation_penalty: f64,
  pub allowed_deviation: f64,
  pub distances:  Vec<Vec<Time>>,
  pub vehicle_definitions: Vec<VehicleDefinition>,
  #[serde(skip_deserializing)]
  pub vehicles: Vec<Vehicle>,
  pub clients: Vec<Client>,
  #[serde(skip)]
  inited: bool,
}

impl Default for ProblemInstance {
  fn default() -> ProblemInstance {
    ProblemInstance {
      name: String::from("Unnamed instance"),
      source: 0,
      deviation_penalty: 0.0,
      allowed_deviation: 0.0,
      distances: vec![],
      vehicle_definitions: vec![],
      vehicles: vec![],
      clients: vec![],
      inited: false,
    }
  }
}

impl ProblemInstance {
  pub fn init(&mut self, optimize_cost: bool) {
    if self.inited {
      return
    }

    self.init_vehicles(optimize_cost);
    self.init_clients();

    self.inited = true;
  }

  /**
   * Initializes vehicles, if optimize cost is false
   * then optimization is over distance, this means
   * that fixed cost is zero and variable cost is 1.
   */
  fn init_vehicles(&mut self, optimize_cost: bool) {
    let mut max: usize = 0;

    let vehicles: Vec<Vehicle> = self.vehicle_definitions.iter().flat_map(|vehicle_def| {
      let min = max;
      max = max + vehicle_def.count as usize;
      (min..max).map(move |id| {
        Vehicle {
          id: id,
          capacity: vehicle_def.capacity,
          fixed_cost: {
            if optimize_cost { vehicle_def.fixed_cost }
            else { 1.0 }
          },
          variable_cost: {
            if optimize_cost { vehicle_def.variable_cost }
            else { 1.0 }
          }
        }
      })
    }).collect();

    self.vehicles = vehicles;
  }

  fn init_clients(&mut self) {
    for index in 0..self.clients.len() {
      self.clients[index].id = index;
    }
  }

  pub fn validate(&self) -> Result<(), String> {
    let node_count = self.clients.len();

    for (index, distances) in self.distances.iter().enumerate() {
      if distances.len() != node_count {
        return Err(format!("Expected distance vector of {} on index {}", node_count, index));
      }
    }

    if self.vehicles.len() == 0 {
      return Err("You must specify at least one vehicle".to_string());
    }

    if self.clients.len() == 0 {
      return Err("You must specify some clients".to_string());
    }

    Ok(())
  }

  ///
  /// Creates a route entry from the following params:
  /// - arc_time: Time to go to the client client_to_id.
  /// - client_to_id: id of the route entry client.
  /// - current_time: time of departure.
  pub fn create_route_entry_client(&self, arc_time: Time, client_to_id: usize, current_time: Time) -> RouteEntryClient {
    let client_to = &self.clients[client_to_id];
    let arrive_time = time_max(current_time + arc_time, client_to.earliest);
    let wait_time = time_max(0 as Time, client_to.earliest - current_time - arc_time);
    let leave_time = arrive_time + client_to.service_time;

    RouteEntryClient {
      client_id: client_to_id,
      arrive_time: arrive_time,
      leave_time: leave_time,
      wait_time: wait_time,
    }
  }

  pub fn compute_route_costs(&self, route: &mut RouteEntry) {
    let vehicle = &self.vehicles[route.vehicle_id];

    route.route_variable_cost = 0 as Cost;
    route.route_fixed_cost = 0 as Cost;
    route.route_time = 0 as Time;
    route.demand = 0.0;

    if route.clients.is_empty() {
      return
    }

    route.route_fixed_cost = vehicle.fixed_cost;

    let mut prev_client_id = route.clients.first().unwrap().client_id;
    for route_client in route.clients.iter() {
      let arc_time = self.distances[prev_client_id][route_client.client_id];

      route.demand += self.clients[route_client.client_id].demand;
      route.route_time += arc_time;
      route.route_variable_cost += arc_time * vehicle.variable_cost;
      prev_client_id = route_client.client_id;
    }
  }

  ///
  /// Check if a move is feasible
  pub fn is_move_feasible(&self, client_from_id: usize, client_to_id: usize, current_time: Time) -> bool {
    let arrival_time = self.distances[client_from_id][client_to_id] + current_time;
    let client = &self.clients[client_to_id];
    
    arrival_time < client.latest + self.allowed_deviation * (client.latest - client.earliest)
  }
  
  ///
  /// Objective calculation
  pub fn evaluate_sol(&self, sol: &mut Solution) {
    let truck_cost = sol.routes.iter().map(|route| route.route_cost()).sum::<Cost>();

    sol.value = truck_cost;
    sol.distance = sol.total_route_time();
  }
}

impl fmt::Display for ProblemInstance {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "  name: {}
  vehicles: {}
  nodes: {}
  allowed excess: {}",
      self.name,
      self.vehicles.len(),
      self.clients.len(),
      self.allowed_deviation,
    )
  }
}
