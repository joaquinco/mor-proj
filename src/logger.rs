
pub mod level {
  pub static DEBUG: i32 = 0;
  pub static INFO: i32 = 1;
  pub static ERROR: i32 = 2; 
}

pub static mut LOG_LEVEL: i32 = level::INFO;

#[allow(dead_code)]
pub fn debug(message: &str) {
  log(message, level::DEBUG);
}

#[allow(dead_code)]
pub fn info(message: &str) {
  log(message, level::INFO);
}

#[allow(dead_code)]
pub fn error(message: &str) {
  log(message, level::ERROR);
}

fn log(message: &str, level: i32) {
  unsafe {
    if LOG_LEVEL > level {
      return;
    }
  }

  println!("{}", message);
}
