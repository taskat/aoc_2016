use std::{collections::HashMap, fmt::Display};

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, input: String) -> String {
        let instructions = create_instructions(input);
        let mut me = Me::new();
        for instruction in instructions {
            me.step(instruction);
        }
        me.pos.distance().to_string()
    }

    fn part_2(&self, input: String) -> String {
        let instructions = create_instructions(input);
        let mut me = Me::new();
        for instruction in instructions {
            if let Some(_) = me.step_2(instruction) {
                break;
            }
        }
        me.pos.distance().to_string()
    }
}

fn create_instructions(data: String) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    let parts = data.split(", ");
    for part in parts {
        instructions.push(Instruction::new(part))
    }
    instructions
}

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

impl Turn {
    fn new(data: Option<char>) -> Turn {
        match data {
            Some('L') => Turn::Left,
            Some('R') => Turn::Right,
            _ => panic!("Inalvid input"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    turn: Turn,
    amount: i32,
}

impl Instruction {
    fn new(data: &str) -> Instruction {
        let mut chars = data.chars();
        let turn = Turn::new(chars.next());
        let amount = chars.collect::<String>().parse().unwrap();
        Instruction {
            turn: turn,
            amount: amount,
        }
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn new() -> Direction {
        Direction::North
    }
    fn turn(&self, t: Turn) -> Direction {
        match *self {
            Direction::North => match t {
                Turn::Left => Direction::West,
                Turn::Right => Direction::East,
            },
            Direction::East => match t {
                Turn::Left => Direction::North,
                Turn::Right => Direction::South,
            },
            Direction::South => match t {
                Turn::Left => Direction::East,
                Turn::Right => Direction::West,
            },
            Direction::West => match t {
                Turn::Left => Direction::South,
                Turn::Right => Direction::North,
            },
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Coord {
    fn new() -> Coord {
        Coord { x: 0, y: 0 }
    }
    fn step(&mut self, amount: i32, dir: &Direction) {
        match dir {
            Direction::North => self.y += amount,
            Direction::East => self.x += amount,
            Direction::South => self.y -= amount,
            Direction::West => self.x -= amount,
        }
    }

    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

struct Me {
    dir: Direction,
    pos: Coord,
    visited: HashMap<Coord, i32>,
}

impl Me {
    fn new() -> Me {
        Me {
            dir: Direction::new(),
            pos: Coord::new(),
            visited: HashMap::new(),
        }
    }

    fn step(&mut self, ins: Instruction) {
        self.dir = self.dir.turn(ins.turn);
        self.pos.step(ins.amount, &self.dir);
    }

    fn step_2(&mut self, ins: Instruction) -> Option<()> {
        self.dir = self.dir.turn(ins.turn);
        for _ in 0..ins.amount {
            if let Some(_) = self.visited.insert(self.pos, 1) {
                return Some(());
            }
            self.pos.step(1, &self.dir);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases = vec![
            (Data::Test(1), "5"),
            (Data::Test(2), "2"),
            (Data::Real, "209"),
        ];
        for case in cases {
            let solution =
                crate::day1::Puzzle {}.part_1(read_input(&FakeConfig::new(1, 1, case.0)).unwrap());
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases = vec![(Data::Test(3), "4"), (Data::Real, "136")];
        for case in cases {
            let solution =
                crate::day1::Puzzle {}.part_2(read_input(&FakeConfig::new(1, 2, case.0)).unwrap());
            assert_eq!(solution, case.1);
        }
    }
}
