use std::any::Any;

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let discs = parse_input(input);
        find_time(discs).to_string()
    }
    fn part_2(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let mut discs = parse_input(input);
        discs.push(Disc::new(11, 0));
        find_time(discs).to_string()
    }
}

#[derive(Debug)]
struct Disc {
    positions: i32,
    start: i32,
}

impl Disc {
    fn new(positions: i32, start: i32) -> Disc {
        Disc { positions, start }
    }
    fn is_at_zero(&self, time: i32) -> bool {
        (self.start + time) % self.positions == 0
    }
}

fn parse_input(input: String) -> Vec<Disc> {
    let mut discs: Vec<Disc> = vec![];
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let positions = parts.nth(3).unwrap().parse::<i32>().unwrap();
        let start = parts
            .last()
            .unwrap()
            .trim_end_matches('.')
            .parse::<i32>()
            .unwrap();
        discs.push(Disc::new(positions, start));
    }
    discs
}

fn find_time(discs: Vec<Disc>) -> i32 {
    let mut time = 0;
    loop {
        let mut fallen = true;
        for (i, disc) in discs.iter().enumerate() {
            if !disc.is_at_zero((time + i + 1).try_into().unwrap()) {
                fallen = false;
                break;
            }
        }
        if fallen {
            return time.try_into().unwrap();
        }
        time += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases: Vec<(Data, &str)> = vec![(Data::Test(1), "5"), (Data::Real, "122318")];
        for case in cases {
            let solution = crate::day15::Puzzle {}
                .part_1(read_input(&FakeConfig::new(15, 1, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Real, "3208583"),
        ];
        for case in cases {
            let solution = crate::day15::Puzzle {}
                .part_2(read_input(&FakeConfig::new(15, 2, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }
}
