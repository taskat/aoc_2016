use std::process;

use common::Config;

use crate::common::EnvConfig;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;

fn main() {
    let config = EnvConfig::new().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let input = common::read_input(&config).unwrap_or_else(|err| {
        println!("Cannot read input file: {}", err);
        process::exit(1);
    });
    let msg = format!("The solution for day {} part {} is: ", config.get_day(), config.get_part());
    let solution = common::solve(input, config);
    println!("{}{}!", msg, solution);
}
