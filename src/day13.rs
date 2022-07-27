use std::{any::Any, collections::HashSet, hash::Hash};

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, input: String, extra_param: Option<Box<dyn Any>>) -> String {
        let number = input.parse::<i32>().unwrap();
        let goal = parse_extra(extra_param, number);
        let start = Coordinate::new(1, 1, number, 0);
        let result = pathfinding::astar(
            &start,
            Coordinate::neighbours,
            |node| node.manhattan_dist(&goal),
            |node| *node == goal,
        );
        result.unwrap().1.to_string()
    }
    fn part_2(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let number = input.parse::<i32>().unwrap();
        let start = Coordinate::new(1, 1, number, 0);
        let mut visited: HashSet<Coordinate> = HashSet::new();
        let mut found: HashSet<Coordinate> = HashSet::new();
        found.insert(start);
        while found.len() != 0 {
            let mut plus: HashSet<Coordinate> = HashSet::new();
            for node in &found {
                for n in node.neighbours() {
                    if n.0.dist <= 50 {
                        plus.insert(n.0);
                    }
                }
            }
            let plus = plus
                .iter()
                .filter(|n| !found.contains(&n) && !visited.contains(&n))
                .map(Coordinate::clone)
                .collect::<Vec<Coordinate>>();
            for node in found {
                visited.insert(node);
            }
            found = plus
                .iter()
                .map(Coordinate::clone)
                .collect::<HashSet<Coordinate>>();
        }
        visited.iter().len().to_string()
    }
}

fn parse_extra(param: Option<Box<dyn Any>>, number: i32) -> Coordinate {
    match param {
        None => Coordinate::new(31, 39, number, 0),
        Some(b) => {
            let data = *b.downcast::<String>().unwrap();
            let mut coords = data.split(" ");
            let x = coords.next().unwrap().parse::<i32>().unwrap();
            let y = coords.next().unwrap().parse::<i32>().unwrap();
            Coordinate::new(x, y, number, 0)
        }
    }
}

#[derive(Clone, Eq)]
struct Coordinate {
    x: i32,
    y: i32,
    is_wall: bool,
    number: i32,
    dist: i32,
}

impl Hash for Coordinate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Coordinate {
    fn new(x: i32, y: i32, number: i32, dist: i32) -> Coordinate {
        let mut c = Coordinate {
            x,
            y,
            is_wall: false,
            number,
            dist,
        };
        c.is_wall = c.compute_is_wall();
        c
    }

    fn compute_is_wall(&self) -> bool {
        let x = self.x;
        let y = self.y;
        let num = x * x + 3 * x + 2 * x * y + y + y * y + self.number;
        num.count_ones() % 2 == 1
    }

    fn neighbours(&self) -> Vec<(Coordinate, i32)> {
        let mut neighbours: Vec<Coordinate> = vec![];
        neighbours.push(Coordinate::new(
            self.x,
            self.y + 1,
            self.number,
            self.dist + 1,
        ));
        neighbours.push(Coordinate::new(
            self.x + 1,
            self.y,
            self.number,
            self.dist + 1,
        ));
        if self.x != 0 {
            neighbours.push(Coordinate::new(
                self.x - 1,
                self.y,
                self.number,
                self.dist + 1,
            ));
        }
        if self.y != 0 {
            neighbours.push(Coordinate::new(
                self.x,
                self.y - 1,
                self.number,
                self.dist + 1,
            ));
        }
        neighbours
            .iter()
            .filter(|neighbour| !neighbour.is_wall)
            .map(|neighbour| (neighbour.clone(), 1 as i32))
            .collect()
    }

    fn manhattan_dist(&self, other: &Coordinate) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
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
            (Data::Test(1), "11", Some(Box::new(String::from("7 4")))),
            (Data::Real, "96", None),
        ];
        for case in cases {
            let solution = crate::day13::Puzzle {}
                .part_1(read_input(&FakeConfig::new(13, 1, case.0)).unwrap(), case.2);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![(Data::Real, "141")];
        for case in cases {
            let solution = crate::day13::Puzzle {}
                .part_2(read_input(&FakeConfig::new(13, 2, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }
}
