use std::{any::Any, collections::HashMap};

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let lines = create_lines(input);
        let mut code: Vec<i32> = vec![];
        let mut current = 5;
        for line in lines {
            current = next_simple(current, line);
            code.push(current);
        }
        code.iter()
            .map(|number| number.to_string())
            .collect::<String>()
    }
    fn part_2(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let lines = create_lines(input);
        let mut code: Vec<char> = vec![];
        let mut current = '5';
        for line in lines {
            current = next_complex(current, line);
            code.push(current);
        }
        code.iter().collect::<String>()
    }
}

fn create_lines(input: String) -> Vec<String> {
    input.split("\r\n").map(|line| line.to_string()).collect()
}

fn next_simple(start: i32, line: String) -> i32 {
    let mut current = start;
    for char in line.chars() {
        match char {
            'U' => {
                if current > 3 {
                    current -= 3
                }
            }
            'R' => {
                if current % 3 > 0 {
                    current += 1
                }
            }
            'D' => {
                if current < 7 {
                    current += 3
                }
            }
            'L' => {
                if current % 3 != 1 {
                    current -= 1
                }
            }
            _ => panic!("Invalid direction"),
        }
    }
    current
}

fn next_complex(start: char, line: String) -> char {
    let mut current = start;
    let neighbors: HashMap<char, HashMap<char, char>> = HashMap::from([
        ('1', HashMap::from([('D', '3')])),
        ('2', HashMap::from([('D', '6'), ('R', '3')])),
        (
            '3',
            HashMap::from([('U', '1'), ('R', '4'), ('D', '7'), ('L', '2')]),
        ),
        ('4', HashMap::from([('D', '8'), ('L', '3')])),
        ('5', HashMap::from([('R', '6')])),
        (
            '6',
            HashMap::from([('U', '2'), ('R', '7'), ('D', 'A'), ('L', '5')]),
        ),
        (
            '7',
            HashMap::from([('U', '3'), ('R', '8'), ('D', 'B'), ('L', '6')]),
        ),
        (
            '8',
            HashMap::from([('U', '4'), ('R', '9'), ('D', 'C'), ('L', '7')]),
        ),
        ('9', HashMap::from([('L', '8')])),
        ('A', HashMap::from([('U', '6'), ('R', 'B')])),
        (
            'B',
            HashMap::from([('U', '7'), ('R', 'C'), ('D', 'D'), ('L', 'A')]),
        ),
        ('C', HashMap::from([('U', '8'), ('L', 'B')])),
        ('D', HashMap::from([('U', 'B')])),
    ]);
    for char in line.chars() {
        current = match neighbors.get(&current).unwrap().get(&char) {
            Some(next) => *next,
            None => current,
        }
    }
    current
}

#[cfg(test)]
mod tests {
    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases = vec![(Data::Test(1), "1985"), (Data::Real, "84452")];
        for case in cases {
            let solution = crate::day2::Puzzle {}
                .part_1(read_input(&FakeConfig::new(2, 1, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases = vec![(Data::Test(1), "5DB3"), (Data::Real, "D65C3")];
        for case in cases {
            let solution = crate::day2::Puzzle {}
                .part_2(read_input(&FakeConfig::new(2, 2, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }
}
