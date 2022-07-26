use std::{env, fmt::Display, fs, io, any::Any};

use crate::{day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11};

#[derive(Copy, Clone)]
pub enum Data {
    Real,
    Test(i32),
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Real => write!(f, "real data"),
            Data::Test(number) => write!(f, "test data {}", number)
        }
    }
}

pub trait Config {
    fn get_day(&self) -> i32;
    fn get_part(&self) -> i32;
    fn get_data_type(&self) -> Data;
}

pub struct EnvConfig {
    day: i32,
    part: i32,
    data_type: Data,
}

impl EnvConfig {
    pub fn new() -> Result<EnvConfig, String> {
        let args: Vec<String> = env::args().collect();
        if args.len() != 4 {
            return Err(String::from("Invalid number of arguments, wanted 4, got ")
                + args.len().to_string().as_str());
        }
        let day = match args[1].parse::<i32>() {
            Ok(day) => day,
            Err(e) => {
                return Err(
                    String::from("Cannot parse second parameter to i32: ") + e.to_string().as_str()
                )
            }
        };
        if day < 1 || day > 25 {
            return Err(String::from("Day parameter must be between 1 and 25"));
        }
        let part = match args[2].parse::<i32>() {
            Ok(part) => part,
            Err(e) => {
                return Err(
                    String::from("Cannot parse third parameter to i32: ") + e.to_string().as_str()
                )
            }
        };
        if part < 1 || part > 2 {
            return Err(String::from("Part parameter must be 1 or 2"));
        }
        let data = match args[3].as_str() {
            "real" => Data::Real,
            s => match s.parse::<i32>() {
                Ok(number) => Data::Test(number),
                Err(e) => return Err(String::from("Fourth parameter must be 'real' or i32: ") + e.to_string().as_str())
            }
        };
        Ok(EnvConfig {
            day: day,
            part: part,
            data_type: data,
        })
    }
}

impl Config for EnvConfig {
    fn get_day(&self) -> i32 {
        self.day
    }

    fn get_part(&self) -> i32 {
        self.part
    }

    fn get_data_type(&self) -> Data {
        self.data_type
    }
}

pub fn read_input(config: &impl Config) -> Result<String, io::Error> {
    let suffix = match config.get_data_type() {
        Data::Real => String::from(""),
        Data::Test(number) => number.to_string(),
    };
    let filepath = format!("inputs/day{}/data{}.txt", config.get_day(), suffix);
    fs::read_to_string(filepath)
}

pub trait Puzzle {
    fn part_1(&self, input: String, extra_param: Option<Box<dyn Any>>) -> String;
    fn part_2(&self, input: String, extra_param: Option<Box<dyn Any>>) -> String;
}

pub fn solve(input: String, config: &EnvConfig) -> String {
    let solver: Box<dyn Puzzle> = match config.day {
        1 => Box::new(day1::Puzzle{}),
        2 => Box::new(day2::Puzzle{}),
        3 => Box::new(day3::Puzzle{}),
        4 => Box::new(day4::Puzzle{}),
        5 => Box::new(day5::Puzzle{}),
        6 => Box::new(day6::Puzzle{}),
        7 => Box::new(day7::Puzzle{}),
        8 => Box::new(day8::Puzzle{}),
        9 => Box::new(day9::Puzzle{}),
        10 => Box::new(day10::Puzzle{}),
        11 => Box::new(day11::Puzzle{}),
        n => panic!("Day {} not implemented yet", n),
    };
    match config.get_part() {
        1 => solver.part_1(input, None),
        2 => solver.part_2(input, None),
        p => panic!("Invalid part {}", p)
    }
}

#[cfg(test)]
pub mod common_test {
    use super::{Config, Data};

    pub struct FakeConfig {
        day: i32,
        part: i32, 
        data_type: Data,
    }

    impl FakeConfig {
        pub fn new(day: i32, part: i32, data_type: Data) -> FakeConfig {
            FakeConfig{day, part, data_type}
        }
    }

    impl Config for FakeConfig {
        fn get_day(&self) -> i32 {
            self.day
        }
    
        fn get_part(&self) -> i32 {
            self.part
        }
    
        fn get_data_type(&self) -> Data {
            self.data_type
        }
    }
}