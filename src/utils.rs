use crate::types::Time;

pub fn time_max(v1: Time, v2: Time) -> Time {
  if v1 > v2 {
    v1
  } else {
    v2
  }
}
