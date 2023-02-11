use std::{any::Any, collections::HashMap};

use crate::common;
use pathfinding;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, _input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let nodes = parse_data(_input);
        let viable_pairs = count_viable_pairs(&nodes);
        viable_pairs.to_string()
    }
    fn part_2(&self, _input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let nodes = parse_data(_input);
        let max_x = nodes.iter().map(|(coord, _)| coord.x).max().unwrap();
        let path = shortest_path(&nodes);
        let steps = path + 1 + (max_x - 1) * 5;
        steps.to_string()
    }
}

fn parse_data(input: String) -> Grid {
    let mut grid = Grid::new();
    for line in input.lines().skip(2) {
        let (coord, data) = Data::parse(line);
        grid.insert(coord, data);
    }
    grid
}

fn count_viable_pairs(grid: &Grid) -> usize {
    let mut count = 0;
    for (coord, data) in grid {
        if data.used == 0 {
            continue;
        }
        for (other_coord, other_data) in grid {
            if coord == other_coord {
                continue;
            }
            if data.used <= other_data.available {
                count += 1;
            }
        }
    }
    count
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Coord { x, y }
    }

    fn manhattan_distance(&self, other: &Coord) -> usize {
        (self.x as isize - other.x as isize).abs() as usize
            + (self.y as isize - other.y as isize).abs() as usize
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Data {
    used: usize,
    available: usize,
}

impl Data {
    fn parse(s: &str) -> (Coord, Self) {
        let mut words = s.split_whitespace().filter(|word| *word != "");
        let mut parts = words.next().unwrap().split('-');
        let x = parts.nth(1).unwrap().trim_start_matches('x').parse().unwrap();
        let y = parts.next().unwrap().trim_start_matches('y').parse().unwrap();
        let coord = Coord { x, y };
        let used = words.nth(1).unwrap().trim_end_matches('T').parse().unwrap();
        let available = words.next().unwrap().trim_end_matches('T').parse().unwrap();
        let data = Data { used, available };
        (coord, data)
    }
}

type Grid = HashMap<Coord, Data>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    coord: Coord,
    data: Data,
}

impl Node {
    fn neighbors(&self, grid: &Grid, max_x: usize, max_y: usize) -> Vec<(Node, usize)> {
        let mut neighbors: Vec<(Node, usize)> = Vec::new();
        if self.coord.x != 0 {
            let c = Coord::new(self.coord.x - 1, self.coord.y);
            neighbors.push((Node{coord: c.clone(), data: grid[&c]}, 1));
        }
        if self.coord.y != 0 {
            let c = Coord::new(self.coord.x, self.coord.y - 1);
            neighbors.push((Node{coord: c.clone(), data: grid[&c]}, 1));
        }
        if self.coord.x != max_x {
            let c = Coord::new(self.coord.x + 1, self.coord.y);
            neighbors.push((Node{coord: c.clone(), data: grid[&c]}, 1));
        }
        if self.coord.y != max_y {
            let c = Coord::new(self.coord.x, self.coord.y + 1);
            neighbors.push((Node{coord: c.clone(), data: grid[&c]}, 1));
        }
        neighbors = neighbors.iter().filter(|(n, _)| n.data.used <= 100).map(|x| *x).collect();
        neighbors
    }
}

fn shortest_path(grid: &Grid) -> usize {
    let empty = grid.iter().find(|(_, data)| data.used == 0).unwrap();
    let start = Node {
        coord: *empty.0,
        data: *empty.1,
    };
    let max_x = grid.iter().map(|(coord, _)| coord.x).max().unwrap();
    let max_y = grid.iter().map(|(coord, _)| coord.y).max().unwrap();
    let goal = Coord::new(max_x - 1, 0);
    let result = pathfinding::astar(&start,
    |n: &Node| n.neighbors(grid, max_x, max_y),
    |n: &Node| n.coord.manhattan_distance(&goal),
    |n: &Node| n.coord == goal);
    result.unwrap().1
}

#[cfg(test)]
mod tests {
    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Test(1), "7"),
            (Data::Real, "1034")
        ];
        for case in cases {
            let solution = crate::day22::Puzzle {}
                .part_1(read_input(&FakeConfig::new(22, 1, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Test(1), "7"),
            (Data::Real, "261")
        ];
        for case in cases {
            let solution = crate::day22::Puzzle {}
                .part_2(read_input(&FakeConfig::new(22, 2, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }
}
