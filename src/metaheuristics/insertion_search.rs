use std::collections::HashSet;

use crate::types::{RouteEntry, RouteEntryClient, ProblemInstance};

use super::local_search::{LocalSearch, LocalSearchNotFound};

struct InsertionError;

/// Try to insert the client_to_insert slice after insert_after on route1
fn try_insert_nodes(
  problem: &ProblemInstance,
  route1: &RouteEntry,
  route2: &RouteEntry,
  insert_after: usize,
  clients_to_insert: &[RouteEntryClient]
) -> Result<(RouteEntry, RouteEntry), InsertionError> {

  let mut new_route1 = route1.clone();
  let mut new_route2 = route2.clone();
  let mut new_route1_demand = 0.0;
  let route1_vehicle = &problem.vehicles[new_route1.vehicle_id];

  new_route1.clients.clear();
  new_route2.clients.clear();

  /* Insert existing previous clients to new route */
  for route_client in &route1.clients[..=insert_after] {
    new_route1.clients.push(route_client.clone());
    new_route1_demand += problem.clients[route_client.client_id].demand;
  }

  /* Closure in charge of checking feasibility of insertions */
  let mut route1_append = |from: &RouteEntryClient, to: &RouteEntryClient| -> Option<RouteEntryClient> {
    if !problem.is_move_feasible(
      from.client_id, to.client_id, from.leave_time
    ) {
      return None
    }
    let client = &problem.clients[to.client_id];
    let distance = problem.distances[from.client_id][to.client_id];

    new_route1_demand += client.demand;

    if new_route1_demand > route1_vehicle.capacity {
      return None
    }

    let ret = problem.create_route_entry_client(
      distance, to.client_id, from.leave_time
    );

    Some(ret)
  };

  let mut current_client = &route1.clients[insert_after];
  /* Insert new clients to new route */
  for route_client in clients_to_insert {
    if let Some(new_route_client) = route1_append(current_client, route_client) {
      new_route1.clients.push(new_route_client);
      current_client = new_route1.clients.last().unwrap();
    } else {
      return Err(InsertionError)
    }
  }

  /* Insert existing posterior clients to new route */
  for route_client in &route1.clients[insert_after + 1..] {
    if let Some(new_route_client) = route1_append(current_client, route_client) {
      new_route1.clients.push(new_route_client);
      current_client = new_route1.clients.last().unwrap();
    } else {
      return Err(InsertionError)
    }
  }

  let moved_clients_ids: HashSet<usize> = clients_to_insert.iter().map(|c| c.client_id).collect();

  /* Creates a new route2 from the clients left */
  let client_from = route2.clients.first().unwrap();
  let mut client_from_id = client_from.client_id;
  let mut current_time = client_from.leave_time;
  for route_client in route2.clients.iter() {
    if !moved_clients_ids.contains(&route_client.client_id) {
      let arc_time = problem.distances[client_from_id][route_client.client_id];
      let new_client = problem.create_route_entry_client(
        arc_time, route_client.client_id, current_time
      );
      current_time = new_client.leave_time;
      client_from_id = new_client.client_id;
      new_route2.clients.push(new_client);
    }
  }

  problem.compute_route_costs(&mut new_route1);
  problem.compute_route_costs(&mut new_route2);

  Ok((new_route1, new_route2))
}

/// Try to move <sequence_length> consecutive clients from route1 to route2.
pub fn insertion_search(
  problem: &ProblemInstance,
  route1: &RouteEntry,
  route2: &RouteEntry,
  sequence_length: usize,
  first_improvement: bool,
) -> Option<(RouteEntry, RouteEntry)> {  
  let ls = LocalSearch::new(first_improvement);

  ls.iterate(&route1.clients, &route2.clients, |index1, c1, index2, c2| {
    let clients_left = route2.clients.len() - index2 - 1;

    if c1.client_id == problem.source || c2.client_id == problem.source || clients_left < sequence_length {
      return Err(LocalSearchNotFound)
    }

    let clients_to_insert = &route2.clients[index2..index2 + sequence_length];
    let insert_result = try_insert_nodes(
      problem, route1, route2, index1, clients_to_insert,
    );
    if let Ok((new_route1, new_route2)) = insert_result {
      let new_value = new_route1.route_cost() + new_route2.route_cost();
      let old_value = route1.route_cost() + route2.route_cost();

      if new_value < old_value {
        return Ok(((new_route1, new_route2), new_value))
      }
    }

    Err(LocalSearchNotFound)
  })
}
