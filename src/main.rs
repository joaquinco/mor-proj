extern crate clap;
extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use clap::{Arg, App, ArgMatches};

mod config;
#[macro_use] mod logger;
mod types;
mod mh;

use config::Config;

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
    .arg(Arg::new("config_file")
      .about("JSON config file")
      .value_name("config.json")
      .required(true)
      .index(1))
    .get_matches()
}

fn parse_config(config_file: &str) -> Result<Config, Box<dyn Error>> {
  debug!("Reading config from {}", config_file);

  let file = File::open(config_file)?;

  let reader = BufReader::new(file);
  let config = serde_json::from_reader(reader)?;

  Ok(config)
}

fn main() {
  let args = parse_args();
  logger::set_level(args.value_of("log_level").unwrap_or("debug"));

  debug!("Starting {}", APP_NAME);

  let config_file = args.value_of("config_file").unwrap();

  let config = match parse_config(config_file) {
    Ok(config) => config,
    Err(e) => panic!(format!("Error reading config file {}", e)),
  };

  debug!("Max iter set to {}", config.iters);

  mh::run(config);
}
