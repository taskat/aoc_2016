use std::{any::Any, collections::{HashMap, HashSet, VecDeque}, vec, hash::Hash};

use itertools::Itertools;

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let graph = create_graph(&input);
        let shortest_path = find_shortest_full_path(graph, false);
        shortest_path.to_string()
    }
    fn part_2(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let graph = create_graph(&input);
        let shortest_path = find_shortest_full_path(graph, true);
        shortest_path.to_string()
    }
}

enum Direction {
    Up,
    Right, 
    Down,
    Left,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    i: i32,
    j: i32,
}

impl Coordinate {
    fn new(i: i32, j: i32) -> Coordinate {
        Coordinate { i, j }
    }

    fn neighbor(&self, direction: Direction) -> Coordinate {
        match direction {
            Direction::Up => Coordinate::new(self.i - 1, self.j),
            Direction::Right => Coordinate::new(self.i, self.j + 1),
            Direction::Down => Coordinate::new(self.i + 1, self.j),
            Direction::Left => Coordinate::new(self.i, self.j - 1),
        }
    }

    fn is_valid(&self, max_i: i32, max_j: i32) -> bool {
        self.i >= 0 && self.j >= 0 && self.i < max_i && self.j < max_j
    }
}

fn find_node(input: &String, node: char) -> Coordinate {
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == node {
                return Coordinate::new(i as i32, j as i32);
            }
        }
    }
    panic!("Node not found");
}

fn find_shortest_paths(input: &String, start: Coordinate) -> HashMap<Coordinate, i32> {
    let mut paths = HashMap::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let max_i = input.lines().count() as i32;
    let max_j = input.lines().nth(0).unwrap().chars().count() as i32;
    queue.push_back((start, 0));
    while !queue.is_empty() {
        let (current, current_distance) = queue.pop_front().unwrap();
        if visited.contains(&current) {
            continue;
        }
        for direction in vec![Direction::Up, Direction::Right, Direction::Down, Direction::Left] {
            let neighbor = current.neighbor(direction);
            if !neighbor.is_valid(max_i, max_j) || visited.contains(&neighbor) {
                continue;
            }
            let neighbor_char = input.lines().nth(neighbor.i as usize).unwrap().chars().nth(neighbor.j as usize).unwrap();
            if neighbor_char == '#' {
                continue;
            }
            if neighbor_char.is_digit(10) {
                if paths.contains_key(&neighbor) {
                    let dist = paths.get(&neighbor).unwrap();
                    if *dist > current_distance + 1 {
                        paths.insert(neighbor, current_distance + 1);
                    }
                } else {
                    paths.insert(neighbor, current_distance + 1);
                }
            }
            queue.push_back((neighbor, current_distance + 1));
        }
        visited.insert(current);
    }
    paths
}

#[derive(Debug)]
struct Node {
    coordinate: Coordinate,
    neighbors: HashMap<i32, i32>
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.coordinate.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.coordinate == other.coordinate
    }
}

impl Eq for Node {}

impl Node {
    fn new(coordinate: Coordinate) -> Node {
        Node {
            coordinate,
            neighbors: HashMap::new()
        }
    }

    fn add_neighbor(&mut self, neighbor_value: i32, distance: i32) {
        self.neighbors.insert(neighbor_value, distance);
    }
}

fn create_graph(input: &String) -> Vec<Node> {
    let mut nodes = Vec::new();
    for i in 0..number_of_nodes(input) {
        let start = find_node(&input, i.to_string().chars().nth(0).unwrap());
        let paths = find_shortest_paths(&input, start);
        let mut start_node = Node::new(start);
        for (coordinate, distance) in paths {
            let value = get_value(input, coordinate);
            start_node.add_neighbor(value, distance);
        }
        nodes.push(start_node);
    }
    nodes
}

fn get_value(input: &String, coordinate: Coordinate) -> i32 {
    input.lines().nth(coordinate.i as usize).unwrap().chars().nth(coordinate.j as usize).unwrap().to_digit(10).unwrap() as i32
}

fn number_of_nodes(input: &String) -> i32 {
    let mut count = 0;
    for line in input.lines() {
        for c in line.chars() {
            if c.is_digit(10) {
                count += 1;
            }
        }
    }
    count
}

fn find_shortest_full_path(graph: Vec<Node>, go_home: bool) -> i32 {
    let values: Vec<i32> = graph.iter().enumerate().map(|(i, _)| i as i32).filter(|i| *i != 0).collect();
    values.iter().permutations(values.len()).map(|permutation| {
        let mut distance = *(&graph[0]).neighbors.get(permutation[0]).unwrap();
        for i in 0..permutation.len() - 1 {
            let current = &graph[*permutation[i] as usize];
            let next = permutation[i + 1];
            distance += current.neighbors.get(next).expect(&next.to_string());
        }
        if go_home {
            distance += graph[0].neighbors.get(permutation[permutation.len() - 1]).unwrap();
        }
        distance
    }).min().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Test(1), "14"),
            (Data::Real, "460"),
        ];
        for case in cases {
            let solution = crate::day24::Puzzle {}
                .part_1(read_input(&FakeConfig::new(24, 1, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Test(1), "20"),
            (Data::Real, "668"),
        ];
        for case in cases {
            let solution = crate::day24::Puzzle {}
                .part_2(read_input(&FakeConfig::new(24, 2, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }
}
