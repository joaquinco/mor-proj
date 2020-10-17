

pub fn remove_from_vec<T: std::cmp::Ord>(list: &mut Vec<T>, elem: &T) {
  match list.binary_search(elem) {
    Ok(index) => { list.remove(index); },
    _ => {}
  };
}
