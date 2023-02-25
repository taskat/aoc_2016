use std::any::Any;

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, _input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let magic_constant = get_magic_constant(_input);
        let mut start = 1;
        let mut odd = true;
        while start < magic_constant {
            if odd {
                start *= 2;
                odd = false;
            } else {
                start = start * 2 + 1;
                odd = true;
            }
        }
        (start - magic_constant).to_string()
    }
    fn part_2(&self, _input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        String::new()
    }
}

fn get_magic_constant(input: String) -> i32 {
    let x = input.lines().nth(1).unwrap().split(" ").nth(1).unwrap().parse::<i32>().unwrap();
    let y = input.lines().nth(2).unwrap().split(" ").nth(1).unwrap().parse::<i32>().unwrap();
    x * y
}
#[cfg(test)]
mod tests {
    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Real, "192")
        ];
        for case in cases {
            let solution = crate::day25::Puzzle {}
                .part_1(read_input(&FakeConfig::new(25, 1, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![];
        for case in cases {
            let solution = crate::day25::Puzzle {}
                .part_2(read_input(&FakeConfig::new(25, 2, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }
}
