use std::any::Any;

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, input: String, extra_param: Option<Box<dyn Any>>) -> String {
        let length = parse_extra(extra_param, 1);
        let mut data = grow(input, length);
        data = data[0..length].to_string();
        checksum(data)
    }

    fn part_2(&self, input: String, extra_param: Option<Box<dyn Any>>) -> String {
        let length = parse_extra(extra_param, 2);
        let mut data = grow(input, length);
        data = data[0..length].to_string();
        checksum(data)
    }
}

fn parse_extra(param: Option<Box<dyn Any>>, part: i32) -> usize {
    match param {
        None => if part == 1 { 272 } else { 35651584 },
        Some(b) => {
            let data = *b.downcast::<String>().unwrap();
            data.parse::<usize>().unwrap()
        }
    }
}

fn grow(mut input: String, length: usize) -> String {
    while input.len() < length {
        let b = input
            .clone()
            .chars()
            .rev()
            .map(|c| if c == '0' { '1' } else { '0' })
            .collect::<String>();
        input = input + "0" + &b;
    }
    input
}

fn checksum(mut data: String) -> String {
    while data.len() % 2 == 0 {
        let odds = data.chars().step_by(2);
        let evens = data.chars().skip(1).step_by(2);
        let pairs = odds.zip(evens);
        data = pairs
            .map(|(a, b)| if a == b { '1' } else { '0' })
            .collect::<String>();
    }
    data
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases: Vec<(Data, &str, Option<Box<dyn Any>>)> = vec![
            (Data::Test(1), "01100", Some(Box::new(String::from("20")))),
            (Data::Real, "11111000111110000", Some(Box::new(String::from("272")))),
        ];
        for case in cases {
            let solution = crate::day16::Puzzle {}
                .part_1(read_input(&FakeConfig::new(16, 1, case.0)).unwrap(), case.2);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str, Option<Box<dyn Any>>)> = vec![
            (Data::Real, "10111100110110100", Some(Box::new(String::from("35651584")))),
        ];
        for case in cases {
            let solution = crate::day16::Puzzle {}
                .part_2(read_input(&FakeConfig::new(16, 2, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }
}
