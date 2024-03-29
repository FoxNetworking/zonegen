use std::{env, process::exit};
use yaml_format::Configuration;

use crate::bind_format::config_to_zone;

mod bind_format;
mod ip_address;
mod yaml_format;

fn main() {
    // TODO(spotlightishere): Switch to clippy or similar for configuration
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Please specify an input YAML filename.");
        exit(1);
    }

    let input_yaml = args.get(1).expect("could not get input YAML filename");
    let input_contents =
        std::fs::read_to_string(input_yaml).expect("unable to read input zone YAML");
    let input: Configuration = serde_yaml::from_str(&input_contents).expect("unable to parse YAML");

    let zone_contents = config_to_zone(input).expect("unable to create zone file");
    println!("{}", zone_contents);
}
