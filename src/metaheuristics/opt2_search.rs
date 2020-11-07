use crate::types::{ProblemInstance, RouteEntry};

///
/// Performs the pseudo 2-OPT local search:
/// Searches for clients with similar time on each route and exchange the route from that point on
fn opt2_search(problem: &ProblemInstance, r1: &RouteEntry, r2: &RouteEntry) -> Option<(RouteEntry, RouteEntry)> {

}
