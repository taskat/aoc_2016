use std::any::Any;

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, _input: String, extra_param: Option<Box<dyn Any>>) -> String {
        let mut start = parse_extra_param(extra_param);
        let operations = parse_operations(&_input);
        operations.iter().for_each(|op| op.execute(&mut start));
        String::from_utf8(start).unwrap()
    }
    fn part_2(&self, _input: String, extra_param: Option<Box<dyn Any>>) -> String {
        let mut start = parse_extra_param(extra_param);
        let operations = parse_operations(&_input);
        operations.iter().rev().for_each(|op| op.undo(&mut start));
        String::from_utf8(start).unwrap()
    }
}

fn parse_extra_param(extra_param: Option<Box<dyn Any>>) -> Vec<u8> {
    let s = *extra_param
        .or(Some(Box::new("abcdefgh")))
        .unwrap()
        .downcast::<String>()
        .unwrap();
    s.as_bytes().to_vec()
}

fn parse_operations(input: &str) -> Vec<Box<dyn Operation>> {
    input
        .lines()
        .map(|line| parse(line))
        .collect::<Vec<Box<dyn Operation>>>()
}

fn parse(input: &str) -> Box<dyn Operation> {
    let words = input.split_whitespace().collect::<Vec<&str>>();
    match words[0] {
        "swap" => match words[1] {
            "position" => SwapPosition::new(words),
            "letter" => SwapLetter::new(words),
            _ => panic!("Unknown swap operation"),
        },
        "rotate" => match words[1] {
            "left" => RotateLeft::new(words),
            "right" => RotateRight::new(words),
            "based" => RotateBased::new(words),
            _ => panic!("Unknown rotate operation"),
        },
        "reverse" => Reverse::new(words),
        "move" => Move::new(words),
        _ => panic!("Unknown operation"),
    }
}

trait Operation {
    fn execute(&self, s: &mut Vec<u8>);
    fn undo(&self, s: &mut Vec<u8>);
}

struct SwapPosition {
    x: usize,
    y: usize,
}

impl SwapPosition {
    fn new(s: Vec<&str>) -> Box<dyn Operation> {
        Box::new(SwapPosition {
            x: s[2].parse::<usize>().unwrap(),
            y: s[5].parse::<usize>().unwrap(),
        })
    }
}

impl Operation for SwapPosition {
    fn execute(&self, s: &mut Vec<u8>) {
        s.swap(self.x, self.y);
    }

    fn undo(&self, s: &mut Vec<u8>) {
        self.execute(s);
    }
}

struct SwapLetter {
    x: u8,
    y: u8,
}

impl SwapLetter {
    fn new(s: Vec<&str>) -> Box<dyn Operation> {
        Box::new(SwapLetter {
            x: s[2].as_bytes()[0],
            y: s[5].as_bytes()[0],
        })
    }
}

impl Operation for SwapLetter {
    fn execute(&self, s: &mut Vec<u8>) {
        let x = s.iter().position(|&c| c == self.x).unwrap();
        let y = s.iter().position(|&c| c == self.y).unwrap();
        s.swap(x, y);
    }

    fn undo(&self, s: &mut Vec<u8>) {
        self.execute(s);
    }
}

struct RotateLeft {
    steps: usize,
}

impl RotateLeft {
    fn new(s: Vec<&str>) -> Box<dyn Operation> {
        Box::new(RotateLeft {
            steps: s[2].parse::<usize>().unwrap(),
        })
    }
}

impl Operation for RotateLeft {
    fn execute(&self, s: &mut Vec<u8>) {
        s.rotate_left(self.steps);
    }

    fn undo(&self, s: &mut Vec<u8>) {
        s.rotate_right(self.steps);
    }
}

struct RotateRight {
    steps: usize,
}

impl RotateRight {
    fn new(s: Vec<&str>) -> Box<dyn Operation> {
        Box::new(RotateRight {
            steps: s[2].parse::<usize>().unwrap(),
        })
    }
}

impl Operation for RotateRight {
    fn execute(&self, s: &mut Vec<u8>) {
        s.rotate_right(self.steps);
    }

    fn undo(&self, s: &mut Vec<u8>) {
        s.rotate_left(self.steps);
    }
}

struct RotateBased {
    x: u8,
}

impl RotateBased {
    fn new(s: Vec<&str>) -> Box<dyn Operation> {
        Box::new(RotateBased {
            x: s[6].as_bytes()[0],
        })
    }
}

impl Operation for RotateBased {
    fn execute(&self, s: &mut Vec<u8>) {
        let x = s.iter().position(|&c| c == self.x).unwrap();
        let steps = 1 + x + if x >= 4 { 1 } else { 0 };
        let len = s.len();
        s.rotate_right(steps % len);
    }

    fn undo(&self, s: &mut Vec<u8>) {
        s.rotate_left(1);
        let mut rotates = 0;
        let mut pos = s.iter().position(|&c| c == self.x).unwrap();
        while pos != rotates {
            s.rotate_left(1);
            rotates += 1;
            if rotates == 4 {
                s.rotate_left(1);
            }
            pos = s.iter().position(|&c| c == self.x).unwrap();
        }
    }
}

struct Reverse {
    x: usize,
    y: usize,
}

impl Reverse {
    fn new(s: Vec<&str>) -> Box<dyn Operation> {
        Box::new(Reverse {
            x: s[2].parse::<usize>().unwrap(),
            y: s[4].parse::<usize>().unwrap(),
        })
    }
}

impl Operation for Reverse {
    fn execute(&self, s: &mut Vec<u8>) {
        s[self.x..=self.y].reverse();
    }

    fn undo(&self, s: &mut Vec<u8>) {
        self.execute(s);
    }
}

struct Move {
    from: usize,
    to: usize,
}

impl Move {
    fn new(s: Vec<&str>) -> Box<dyn Operation> {
        Box::new(Move {
            from: s[2].parse::<usize>().unwrap(),
            to: s[5].parse::<usize>().unwrap(),
        })
    }
}

impl Operation for Move {
    fn execute(&self, s: &mut Vec<u8>) {
        let c = s.remove(self.from);
        s.insert(self.to, c);
    }

    fn undo(&self, s: &mut Vec<u8>) {
        let c = s.remove(self.to);
        s.insert(self.from, c);
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases: Vec<(Data, &str, Option<Box<dyn Any>>)> = vec![
            (Data::Test(1), "decab", Some(Box::new(String::from("abcde")))),
            (Data::Real, "bfheacgd", Some(Box::new(String::from("abcdefgh"))))
        ];
        for case in cases {
            let solution = crate::day21::Puzzle {}
                .part_1(read_input(&FakeConfig::new(21, 1, case.0)).unwrap(), case.2);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str, Option<Box<dyn Any>>)> = vec![
            (Data::Real, "gcehdbfa", Some(Box::new(String::from("fbgdceah"))))
        ];
        for case in cases {
            let solution = crate::day21::Puzzle {}
                .part_2(read_input(&FakeConfig::new(21, 2, case.0)).unwrap(), case.2);
            assert_eq!(solution, case.1);
        }
    }
}
