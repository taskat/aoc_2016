use std::{any::Any, fmt::Display};

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let mut screen = Screen::new(6, 50);
        let instructions = create_instructions(input);
        for instruction in instructions {
            (*instruction).execute(&mut screen);
        }
        screen.count().to_string()
    }
    fn part_2(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let mut screen = Screen::new(6, 50);
        let instructions = create_instructions(input);
        for instruction in instructions {
            (*instruction).execute(&mut screen);
        }
        println!("{}", screen);
        String::new()
    }
}

struct Screen {
    display: Vec<Vec<bool>>,
}

impl Screen {
    fn new(row: usize, col: usize) -> Screen {
        Screen {
            display: vec![vec![false; col]; row],
        }
    }

    fn rect(&mut self, rect: &Rect) {
        for i in 0..rect.b {
            for j in 0..rect.a {
                self.display[i][j] = true;
            }
        }
    }

    fn rotate_row(&mut self, rotate: &RotateRow) {
        self.display[rotate.row].rotate_right(rotate.amount);
    }

    fn rotate_column(&mut self, rotate: &RotateColumn) {
        let mut original = vec![];
        for line in &self.display {
            original.push(line[rotate.col]);
        }
        original.rotate_right(rotate.amount);
        for (i, value) in original.iter().enumerate() {
            self.display[i][rotate.col] = *value;
        }
    }

    fn count(&self) -> usize {
        self.display
            .iter()
            .map(|line| line.iter().filter(|value| **value).count())
            .sum()
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines: Vec<String> = vec![];
        for row in &self.display {
            let mut line = String::new();
            for pixel in row {
                match pixel {
                    true => line += "#",
                    false => line += ".",
                }
            }
            lines.push(line)
        }
        write!(f, "{}", lines.join("\n"))
    }
}

fn create_instructions(input: String) -> Vec<Box<dyn Instruction>> {
    let lines = input.split("\r\n");
    let mut instructions: Vec<Box<dyn Instruction>> = vec![];
    for line in lines {
        if line.contains("rect") {
            instructions.push(Box::new(Rect::new(line.to_string())));
        }
        if line.contains("row") {
            instructions.push(Box::new(RotateRow::new(line.to_string())));
        }
        if line.contains("column") {
            instructions.push(Box::new(RotateColumn::new(line.to_string())));
        }
    }
    instructions
}

trait Instruction {
    fn execute(&self, screen: &mut Screen);
}

struct Rect {
    a: usize,
    b: usize,
}

impl Rect {
    fn new(line: String) -> Rect {
        let mut numbers = line
            .split("x")
            .map(|word| word.trim_matches(|c| !char::is_numeric(c)).parse::<usize>())
            .filter(|result| result.is_ok())
            .map(|result| result.unwrap());
        let a: usize = numbers.next().unwrap();
        let b: usize = numbers.next().unwrap();
        Rect { a, b }
    }
}

impl Instruction for Rect {
    fn execute(&self, screen: &mut Screen) {
        screen.rect(&self);
    }
}

struct RotateRow {
    row: usize,
    amount: usize,
}

impl RotateRow {
    fn new(line: String) -> RotateRow {
        let mut numbers = line
            .split(" ")
            .map(|word| word.trim_matches(|c| !char::is_numeric(c)).parse::<usize>())
            .filter(|result| result.is_ok())
            .map(|result| result.unwrap());
        let row: usize = numbers.next().unwrap();
        let amount: usize = numbers.next().unwrap();
        RotateRow { row, amount }
    }
}

impl Instruction for RotateRow {
    fn execute(&self, screen: &mut Screen) {
        screen.rotate_row(self);
    }
}

struct RotateColumn {
    col: usize,
    amount: usize,
}

impl RotateColumn {
    fn new(line: String) -> RotateColumn {
        let mut numbers = line
            .split(" ")
            .map(|word| word.trim_matches(|c| !char::is_numeric(c)).parse::<usize>())
            .filter(|result| result.is_ok())
            .map(|result| result.unwrap());
        let col: usize = numbers.next().unwrap();
        let amount: usize = numbers.next().unwrap();
        RotateColumn { col, amount }
    }
}

impl Instruction for RotateColumn {
    fn execute(&self, screen: &mut Screen) {
        screen.rotate_column(self);
    }
}

#[cfg(test)]
mod tests {
    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases: Vec<(Data, &str)> = vec![(Data::Test(1), "6"), (Data::Real, "116")];
        for case in cases {
            let solution = crate::day8::Puzzle {}
                .part_1(read_input(&FakeConfig::new(8, 1, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![];
        for case in cases {
            let solution = crate::day8::Puzzle {}
                .part_2(read_input(&FakeConfig::new(8, 2, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }
}
