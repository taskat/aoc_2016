use std::any::Any;

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, input: String, extra_param: Option<Box<dyn Any>>) -> String {
        let rows = parse_extra_param(extra_param, 1);
        let safe_tiles = count_safe_tiles(input, rows);
        safe_tiles.to_string()
    }
    fn part_2(&self, input: String, extra_param: Option<Box<dyn Any>>) -> String {
        let rows = parse_extra_param(extra_param, 2);
        let safe_tiles = count_safe_tiles(input, rows);
        safe_tiles.to_string()
    }
}

fn parse_extra_param(extra_param: Option<Box<dyn Any>>, part: i32) -> i32 {
    match extra_param {
        Some(param) => match str::parse::<i32>(&param.downcast::<String>().unwrap()) {
            Ok(param) => param,
            Err(_) => panic!("Invalid extra param"),
        },
        None => if part == 1 { 40 } else { 400000 },
    }
}

fn next_line(line: String) -> String {
    let mut traps: Vec<bool> = line.chars().map(|c| c == '^').collect();
    traps.insert(0, false);
    traps.push(false);
    traps.windows(3)
        .map(|w| prev_traps(w[0], w[1], w[2]))
        .map(|trap| if trap { '^' } else { '.' })
        .collect()
}

fn prev_traps(left: bool, center: bool, right: bool) -> bool {
    (left && center && !right)
        || (!left && center && right)
        || (left && !center && !right)
        || (!left && !center && right)
}

fn count_safe_tiles(first_line: String, rows: i32) -> i32 {
    let mut safe_tiles = count_safe_tiles_in_line(&first_line);
    let mut line = next_line(first_line);
    for _i in 1..rows {
        safe_tiles += count_safe_tiles_in_line(&line);
        line = next_line(line);
    }
    safe_tiles
}

fn count_safe_tiles_in_line(line: &String) -> i32 {
    line.chars().filter(|c| *c == '.').count() as i32
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases: Vec<(Data, &str, Option<Box<dyn Any>>)> = vec![
            (Data::Test(1), "6", Some(Box::new(String::from("3")))),
            (Data::Test(2), "38", Some(Box::new(String::from("10")))),
            (Data::Real, "1978", Some(Box::new(String::from("40"))))
        ];
        for case in cases {
            let solution = crate::day18::Puzzle {}
                .part_1(read_input(&FakeConfig::new(18, 1, case.0)).unwrap(), case.2);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str, Option<Box<dyn Any>>)> = vec![
            (Data::Real, "20003246", Some(Box::new(String::from("400000"))))
        ];
        for case in cases {
            let solution = crate::day18::Puzzle {}
                .part_2(read_input(&FakeConfig::new(18, 2, case.0)).unwrap(), case.2);
            assert_eq!(solution, case.1);
        }
    }
}
