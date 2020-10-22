use std:error:Error;

#[derive(Debug)]
struct MhError {
  reason: String,
};

impl Error for MhError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    Some(&self.reason)
  }
}
