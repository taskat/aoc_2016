use std::{any::Any, collections::HashSet};

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let start = State::new(0, 0, &input);
        let full_path = find_best_path(start);
        full_path.replace(&input, "")
    }
    fn part_2(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let start = State::new(0, 0, &input);
        find_longest_path(start).to_string()
    }
}

fn find_best_path(start: State) -> String {
    let result = pathfinding::astar(
        &start,
        |state| state.neighbors(),
        |state| state.manhatten_distance(),
        is_vault,
    )
    .unwrap();
    let last = result.0.last().unwrap();
    last.path.clone()
}

fn find_longest_path(start: State) -> i32 {
    let base_length = start.path.len() as i32;
    let mut states = HashSet::new();
    states.insert(start);
    let mut longest = 0;
    let mut new_states = HashSet::new();
    while !states.is_empty() {
        for state in states {
            let neighbors = state.neighbors();
            for neighbor in neighbors {
                if is_vault(&neighbor.0) {
                    longest = longest.max(neighbor.0.path.len() as i32);
                } else {
                    new_states.insert(neighbor.0);
                }
            }
        }
        states = new_states;
        new_states = HashSet::new();
    }
    longest - base_length
}

fn is_vault(state: &State) -> bool {
    state.x == 3 && state.y == 3
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(x: usize) -> Direction {
        match x {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    x: i32,
    y: i32,
    path: String,
}

impl State {
    fn new(x: i32, y: i32, path: &String) -> State {
        State {
            x,
            y,
            path: path.clone(),
        }
    }

    fn open_doors(&self) -> Vec<Direction> {
        let mut doors = Vec::new();
        let hash = md5::compute(self.path.as_str());
        let hash = format!("{:x}", hash);
        for i in 0..4 {
            let c = hash.chars().nth(i).unwrap();
            if c >= 'b' && c <= 'f' {
                doors.push(Direction::new(i));
            }
        }
        doors
    }

    fn neighbors(&self) -> Vec<(State, i32)> {
        let mut neighbors = Vec::new();
        for direction in self.open_doors() {
            let mut new_state = self.clone();
            match direction {
                Direction::Up => new_state.y -= 1,
                Direction::Down => new_state.y += 1,
                Direction::Left => new_state.x -= 1,
                Direction::Right => new_state.x += 1,
            }
            if new_state.x < 0 || new_state.x > 3 || new_state.y < 0 || new_state.y > 3 {
                continue;
            }
            new_state.path.push(match direction {
                Direction::Up => 'U',
                Direction::Down => 'D',
                Direction::Left => 'L',
                Direction::Right => 'R',
            });
            neighbors.push((new_state, 1));
        }
        neighbors
    }

    fn manhatten_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[cfg(test)]
mod tests {
    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Test(1), "DDRRRD"),
            (Data::Test(2), "DDUDRLRRUDRD"),
            (Data::Test(3), "DRURDRUDDLLDLUURRDULRLDUUDDDRR"),
            (Data::Real, "RDURRDDLRD"),
        ];
        for case in cases {
            let solution = crate::day17::Puzzle {}
                .part_1(read_input(&FakeConfig::new(17, 1, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Test(1), "370"),
            (Data::Test(2), "492"),
            (Data::Test(3), "830"),
            (Data::Real, "526"),
        ];
        for case in cases {
            let solution = crate::day17::Puzzle {}
                .part_2(read_input(&FakeConfig::new(17, 2, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }
}
