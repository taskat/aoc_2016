use std::process;

use common::Config;

use crate::common::EnvConfig;

mod common;
mod day1;
mod day2;
mod day3;

fn main() {
    let config = EnvConfig::new().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let input = common::read_input(&config).unwrap_or_else(|err| {
        println!("Cannot read input file: {}", err);
        process::exit(1);
    });
    let solution = common::solve(input, &config);
    println!("The solution for day {} part {} is: {}!", config.get_day(), config.get_part(), solution)
}
