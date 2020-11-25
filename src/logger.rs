use chrono::{DateTime, Local};

pub mod level {
  pub static DEBUG: i32 = 0;
  pub static INFO: i32 = 1;
  pub static ERROR: i32 = 2;

  pub fn level_name(level: i32) -> &'static str {
    if level == DEBUG {
      "DEBUG"
    } else if level == INFO {
      "INFO"
    } else if level == ERROR {
      "ERROR"
    } else {
      "UNKOWN"
    }
  }
}



pub static mut LOG_LEVEL: i32 = level::INFO;

#[allow(dead_code)]
pub fn set_level(level: &str) {
  unsafe {
    LOG_LEVEL = match level.to_string().to_lowercase().as_ref() {
      "debug" => { level::DEBUG }
      "info" => { level::INFO }
      "error" => { level::ERROR }
      _ => LOG_LEVEL
    }
  }
}

#[allow(dead_code)]
pub fn debug(message: String) {
  log(message, level::DEBUG);
}

#[allow(dead_code)]
pub fn info(message: String) {
  log(message, level::INFO);
}

#[allow(dead_code)]
pub fn error(message: String) {
  log(message, level::ERROR);
}

fn log(message: String, level: i32) {
  unsafe {
    if LOG_LEVEL > level {
      return;
    }
  }

  let now: DateTime<Local> = Local::now();
  let level_name = level::level_name(level);
  println!("{} | {} | {}", now, level_name, message);
}

#[macro_export]
macro_rules! debug {
  ($($arg:tt)*) => {
    crate::logger::debug(format!($($arg)*))
  }
}

#[macro_export]
macro_rules! info {
  ($($arg:tt)*) => {
    crate::logger::info(format!($($arg)*))
  }
}

#[macro_export]
macro_rules! error {
  ($($arg:tt)*) => {
    crate::logger::error(format!($($arg)*))
  }
}
