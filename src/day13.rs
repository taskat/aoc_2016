use std::any::Any;

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, _input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        String::new()
    }
    fn part_2(&self, _input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases: Vec<(Data, &str)> = vec![];
        for case in cases {
            let solution = crate::day13::Puzzle {}
                .part_1(read_input(&FakeConfig::new(13, 1, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![];
        for case in cases {
            let solution = crate::day13::Puzzle {}
                .part_2(read_input(&FakeConfig::new(13, 2, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }
}
