extern crate clap;

use clap::{Arg, App, ArgMatches};

mod types;
mod logger;

static APP_NAME: &str = "mor-proj";


fn parse_args() -> ArgMatches {
  App::new(APP_NAME)
    .version("1.0")
    .author("Joaquín Correa <joaquin.correa@fing.edu.uy>")
    .arg(Arg::new("log_level")
      .short('l')
      .long("log-level")
      .value_name("debug|info|error")
      .about("Sets the log level")
      .takes_value(true))
    .arg(Arg::new("config")
      .about("Sets the config file to use")
      .required(true)
      .index(1))
    .get_matches()
}

fn main() {
  let args = parse_args();
  logger::set_level(args.value_of("log_level").unwrap_or("debug"));

  logger::debug(&format!("Starting {}", APP_NAME));
}
