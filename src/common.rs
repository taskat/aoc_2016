use std::{env, fmt::Display, fs, io};


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

pub struct Config {
    day: i32,
    part: i32,
    data: Data,
}

impl Config {
    pub fn get_day(&self) -> i32 {
        self.day
    }
    pub fn get_part(&self) -> i32 {
        self.part
    }
    pub fn new() -> Result<Config, String> {
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
        Ok(Config {
            day: day,
            part: part,
            data: data,
        })
    }
}

pub fn read_input(config: &Config) -> Result<String, io::Error> {
    let suffix = match config.data {
        Data::Real => String::from(""),
        Data::Test(number) => number.to_string(),
    };
    let filepath = format!("inputs/day{}/data{}.txt", config.day, suffix);
    fs::read_to_string(filepath)
}

pub trait Puzzle {
    fn part_1(&self, input: String) -> String;
    fn part_2(&self, input: String) -> String;
}

pub fn solve(input: String, config: &Config) -> String {
    let solver = match config.day {
        n => panic!("Day {} not implemented yet", n),
    };
    match config.get_part() {
        1 => solver.part_1(input),
        2 => solver.part_2(input),
        p => panic!("Invalid part {}", p)
    }
}