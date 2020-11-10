use crate::types::{ProblemInstance, RouteEntry, Time};

use crate::utils::time_max;

///
/// Check if a subroute can be inserted within a route at client_index arriving at that
/// client at new_arrival_time.
fn is_subroute_feasible(
  problem:&ProblemInstance,
  route: &RouteEntry,
  client_index: usize,
  new_arrival_time: Time
) -> bool {
  let mut current_time = new_arrival_time;
  let mut prev_client_id = route.clients[client_index].client_id;
  let mut curr_demand: f64 = route.clients[..=client_index]
                         .iter()
                         .map(|route| problem.clients[route.client_id].demand).sum();
  let vehicle = &problem.vehicles[route.vehicle_id];

  for client_route in route.clients[client_index + 1..].iter() {
    let client_id = client_route.client_id;
    let client = &problem.clients[client_id];
    curr_demand += client.demand;

    if curr_demand > vehicle.capacity {
      return false
    }

    if !problem.is_move_feasible(prev_client_id, client_id, current_time) {
      return false
    }

    let arc_time = problem.distances[prev_client_id][client_id];
    current_time = time_max(current_time + arc_time, client.earliest) + client.service_time;
    prev_client_id = client_id;
  }

  true
}

fn replace_subroute(
  problem: &ProblemInstance,
  route: &mut RouteEntry,
  source: &RouteEntry,
  route_index: usize,
  source_index: usize,
) {
  for _ in route_index..route.clients.len() {
    route.clients.pop();
  }
  
  let mut prev_client_id;
  let mut current_time;

  if route.clients.is_empty() {
    prev_client_id = problem.source;
    current_time = problem.clients[prev_client_id].earliest;
  } else {
    let route_client = route.clients.last().unwrap();
    prev_client_id = route_client.client_id;
    current_time = route_client.leave_time;
  }

  for route_client in source.clients[source_index..].iter() {
    let client_id = route_client.client_id;
    let arc_time = problem.distances[prev_client_id][client_id];

    let new_route_client = problem.create_route_entry_client(
      arc_time, client_id, current_time
    );
    current_time = new_route_client.leave_time;
    prev_client_id = client_id;
    route.clients.push(new_route_client);
  }

  problem.compute_route_costs(route);
}

///
/// Exchange subroutes between RouteEntries by creating new routes
fn exchange_subroutes(
  problem: &ProblemInstance,
  route1: &RouteEntry,
  route2: &RouteEntry,
  client1_index: usize,
  client2_index: usize,
) -> (RouteEntry, RouteEntry) {
  let mut new_route1 = route1.clone();
  let mut new_route2 = route2.clone();

  replace_subroute(problem, &mut new_route1, route2, client1_index, client2_index);
  replace_subroute(problem, &mut new_route2, route1, client2_index, client1_index);

  (new_route1, new_route2)
}

///
/// Performs the pseudo 2-OPT local search:
/// Searches for clients with similar time on each route and exchange the route from that point on.
pub fn opt2_search(
  problem: &ProblemInstance,
  route1: &RouteEntry,
  route2: &RouteEntry,
  first_improvement: bool,
) -> Option<(RouteEntry, RouteEntry)> {
  let mut ret: Option<(RouteEntry, RouteEntry)> = None;

  /* Checks if transformations c1->next(c2) c2->next(c1) are
   * feasible and computes the new solution cost if applies.
   * It assumes route are at least three clients length.
   */
  for (index1, c1) in route1.clients.iter().enumerate() {
    for (index2, c2) in route2.clients.iter().enumerate() {
      if c1.client_id == problem.source || c2.client_id == problem.source {
        continue
      }

      let next_c1 = &route1.clients[index1 + 1];
      let next_c2 = &route2.clients[index2 + 1];

      let arrival_next_c1 = problem.distances[c2.client_id][next_c1.client_id] + c2.leave_time;
      let arrival_next_c2 = problem.distances[c1.client_id][next_c2.client_id] + c1.leave_time;

      let exchange_feasible = {
        problem.is_move_feasible(c1.client_id, next_c2.client_id, c1.leave_time)
        &&
        problem.is_move_feasible(c2.client_id, next_c1.client_id, c2.leave_time)
        &&
        is_subroute_feasible(problem, &route1, index1 + 1, arrival_next_c1)
        &&
        is_subroute_feasible(problem, &route2, index2 + 1, arrival_next_c2)
      };

      if !exchange_feasible {
        continue;
      }
      let (new_route1, new_route2) = exchange_subroutes(problem, route1, route2, index1, index2);

      if new_route1.route_cost() + new_route2.route_cost() < route1.route_cost() + route2.route_cost() {
        ret = Some((new_route1, new_route2));
      }

      if ret.is_some() && first_improvement {
        return ret;
      }
    }
  }

  ret
}
