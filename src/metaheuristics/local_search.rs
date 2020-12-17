use std::iter::Iterator;

pub struct LocalSearch {
  pub first_improvement: bool,
}

pub struct LocalSearchNotFound;

impl LocalSearch {
  pub fn new(first_improvement: bool) -> Self {
    Self { first_improvement: first_improvement }
  }

  pub fn iterate<T, S, Func: Fn(usize, &T, usize, &T) -> Result<(S, f64), LocalSearchNotFound>>(
    &self, values1: &Vec<T>, values2: &Vec<T>, search: Func
  ) -> Option<S> {
    let mut ret: Option<S> = None;
    let mut best_value: f64 = 0.0;
    let mut solutions_found: usize = 0;
    let mut first_value: Option<f64> = None;
      
    for (index1, v1) in values1.iter().enumerate() {
      for (index2, v2) in values2.iter().enumerate() {
        let result = search(index1, v1, index2, v2);
        if let Ok((current_sol, current_value)) = result {
          if current_value < best_value || ret.is_none() {
            solutions_found += 1;
            ret = Some(current_sol);

            if self.first_improvement {
              return ret;
            }

            if first_value.is_none() {
              first_value = Some(current_value);
            }

            best_value = current_value;
          }
        }
      }
    }

    if ret.is_some() {
      debug!(
        "LocalSearch best, first, explored: {}, {}, {}",
        best_value, first_value.unwrap(), solutions_found
      );
    }

    ret
  }
}
