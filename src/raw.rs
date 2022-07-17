use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, _input: String) -> String {
        String::new()
    }
    fn part_2(&self, _input: String) -> String {
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
            let solution =
                crate::day__::Puzzle {}.part_1(read_input(&FakeConfig::new(__, 1, case.0)).unwrap());
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![];
        for case in cases {
            let solution =
                crate::day__::Puzzle {}.part_2(read_input(&FakeConfig::new(__, 2, case.0)).unwrap());
            assert_eq!(solution, case.1);
        }
    }
}