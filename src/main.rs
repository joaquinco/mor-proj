mod types;
mod logger;

use std::env;

fn parse_args() -> App {

}

fn main() {
    let args: Vec<String> = env::args().collect();
    logger::debug(&format!("Starting {}", args[0]));

    logger
}
