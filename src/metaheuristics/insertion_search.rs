use crate::types::{RouteEntry, ProblemInstance};
use super::local_search::{LocalSearch, LocalSearchNotFound};

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
    Err(LocalSearchNotFound)
  })
}
