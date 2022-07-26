use std::{
    any::Any,
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut}, collections::hash_map::DefaultHasher,
};

use crate::common;
use pathfinding;

const FLOORS: usize = 4;

pub struct Puzzle {}

fn test() {
    let a = Area::new(String::from("The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
    The second floor contains a hydrogen generator.
    The third floor contains a lithium generator.
    The fourth floor contains nothing relevant."));
    let b = Area::new(String::from("The first floor contains a lithium-compatible microchip and a hydrogen-compatible microchip.
    The second floor contains a hydrogen generator.
    The third floor contains a lithium generator.
    The fourth floor contains nothing relevant."));
    let mut hasher = DefaultHasher::new();
    a.hash(&mut hasher);
    println!("{}", hasher.finish());
    hasher = DefaultHasher::new();
    b.hash(&mut hasher);
    println!("{}", hasher.finish());
}

impl common::Puzzle for Puzzle {
    fn part_1(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let start = Area::new(input);
        println!("{:?}", start);
        //test();
        let result = pathfinding::astar(
            &start,
            Area::neighbours,
            Area::heuristic_cost,
            Area::finished
        );
        result.unwrap().1.to_string()
    }
    fn part_2(&self, _input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        String::new()
    }
}

#[derive(Eq)]
struct Area {
    floors: [Floor; FLOORS],
    current: usize,
    c: i64
}

impl PartialEq for Area {
    fn eq(&self, other: &Self) -> bool {
        if self.current != other.current {
            return false;
        }
        for i in 0..FLOORS {
            if self.floors[i].len() != other.floors[i].len() {
                return false;
            }
            for item in self.floors[i].iter() {
                if !other.floors[i].contains(&item) {
                    return false;
                }
            }
        }
        true
    }
}

impl Hash for Area {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..FLOORS {
            self.floors[i].hash(state);
        }
        self.current.hash(state);
    }
}

impl Clone for Area {
    fn clone(&self) -> Self {
        Self {
            floors: self.floors.iter().map(Floor::clone).collect::<Vec<Floor>>().try_into().unwrap(),
            ..*self
        }
    }
}

impl Area {
    fn new(input: String) -> Area {
        let floors: [Floor; 4] = input.split("\r\n")
            .map(Floor::new)
            .collect::<Vec<Floor>>()
            .try_into()
            .unwrap();
        Area {
            floors,
            current: 0,
            c: 0
        }
    }

    fn move_item(&mut self, from: usize, to: usize, item: &Item) {
        self.c += 1;
        let item = self.floors[from].remove(item);
        self.floors[to].add(item);
    }

    fn neighbours(&self) -> Vec<(Area, usize)> {
        if self.c % 100 == 0 {
            println!("{} {}",self.c, self.heuristic_cost());
        }
        let mut neighbors: Vec<(Area, usize)> = vec![];
        let valid_items = self.floors[self.current].get_valid_items();
        for valid in &valid_items {
            let mut clone = self.clone();
            match self.current {
                0 => {
                    for v in valid {
                        clone.move_item(0, 1, v);
                    }
                    clone.current = 1;
                }
                3 => {
                    for v in valid {
                        clone.move_item(3, 2, v);
                    }
                    clone.current = 2
                }
                i => {
                    for v in valid {
                        clone.move_item(i, i - 1, v);
                    }
                    clone.current = i - 1;
                    neighbors.push((clone, 1));
                    clone = self.clone();
                    for v in valid {
                        clone.move_item(i, i + 1, v);
                    }
                    clone.current = i + 1;
                }
            }
            neighbors.push((clone, 1));
        }
        neighbors
            .iter()
            .filter(|(neighbour, _)| neighbour.is_valid())
            .filter(|(neighbour, _)| neighbour.optimize())
            .map(|neighbour| neighbour.clone())
            .collect()
    }

    fn optimize(&self) -> bool {
        if self.current == 0 || self.current == FLOORS - 1 {
            if self.floors[self.current].len() == 1 {
                return false;
            }
        }
        true
    }

    fn is_valid(&self) -> bool {
        self.floors.iter().all(Floor::is_valid)
    }

    fn heuristic_cost(&self) -> usize {
        let mut cost = 0;
        for i in 0..FLOORS {
            cost += self.floors[i].len() * (FLOORS - i - 1);
        }
        cost
    }

    fn finished(&self) -> bool {
        let mut floors = self.floors.iter().rev();
        floors.next();
        floors.all(|floor| floor.len() == 0)
    }
}

impl Debug for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = self
            .floors
            .iter()
            .map(|items| {
                items
                    .iter()
                    .map(|item| format!("{:?}", item))
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>();
        for i in 0..lines.len() {
            lines[i] = format!(
                "F{} {} {}",
                i + 1,
                if i == self.current as usize { "E" } else { " " },
                lines[i]
            );
        }
        lines.reverse();
        write!(f, "{}", lines.join("\n"))
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Floor(Vec<Item>);

impl Deref for Floor {
    type Target = Vec<Item>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Floor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Floor {
    fn new(line: &str) -> Floor {
        let mut floor = Floor(vec![]);
        let mut words = line.split(' ').rev();
        while let Some(word) = words.next() {
            match word {
                "generator" | "generator," | "generator." => {
                    floor.push(Item::new(words.next().unwrap().to_string(), true))
                }
                "microchip" | "microchip," | "microchip." => floor.push(Item::new(
                    words.next().unwrap().replace("-compatible", "").to_string(),
                    false,
                )),
                _ => (),
            }
        }
        println!("{:?}", floor);
        floor.sort();
        floor
    }
    
    fn get_valid_items(&self) -> Vec<Vec<Item>> {
        let mut result: Vec<Vec<Item>> = vec![];
        if self.len() == 0 {
            return result;
        }
        let generators = self.number_of_generators();
        //1 item taken
        for item in &self.0 {
            match item {
                Item::Microchip(_) => result.push(Vec::from([item.clone()])),
                Item::Generator(e) => {
                    if generators == 1 || !self.has_chip(e) {
                        result.push(Vec::from([item.clone()]))
                    }
                }
            }
        }
        //2 items taken
        for i in 0..self.len() - 1 {
            for j in i + 1..self.len() {
                match (&self[i], &self[j]) {
                    (Item::Microchip(_), Item::Microchip(_)) => {
                        result.push(Vec::from([self[i].clone(), self[j].clone()]))
                    }
                    (Item::Microchip(em), Item::Generator(eg))
                    | (Item::Generator(eg), Item::Microchip(em)) => {
                        if eg == em {
                            result.push(Vec::from([self[i].clone(), self[j].clone()]));
                            continue;
                        }
                    }
                    (Item::Generator(e1), Item::Generator(e2)) => {
                        if generators == 2 || (!self.has_chip(&e1) && !self.has_chip(&e2)) {
                            result.push(Vec::from([self[i].clone(), self[j].clone()]))
                        }
                    }
                }
            }
        }
        result
    }

    fn has_chip(&self, element: &String) -> bool {
        self.iter()
            .find(|item| {
                if let Item::Microchip(elem) = item {
                    return elem == element;
                }
                false
            })
            .is_some()
    }

    fn has_generator(&self, element: &String) -> bool {
        self.iter()
            .find(|item| {
                if let Item::Generator(elem) = item {
                    return elem == element;
                }
                false
            })
            .is_some()
    }

    fn number_of_generators(&self) -> usize {
        self.iter()
            .filter(|item| match item {
                Item::Generator(_) => true,
                Item::Microchip(_) => false,
            })
            .count()
    }

    fn remove(&mut self, item_to_remove: &Item) -> Item {
        let idx = self
            .0
            .iter()
            .position(|item| item == item_to_remove)
            .unwrap();
        let removed = self.0.remove(idx);
        self.sort();
        removed
    }

    fn add(&mut self, item: Item) {
        self.push(item);
        self.sort();
    }

    fn is_valid(&self) -> bool {
        if self.number_of_generators() == 0 {
            return true;
        }
        self.iter()
            .filter(|item| match item {
                Item::Generator(_) => false,
                Item::Microchip(_) => true,
            })
            .all(|chip| self.has_generator(chip.get_elem()))
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
enum Item {
    Microchip(String),
    Generator(String),
}

impl Item {
    fn new(elem: String, generator: bool) -> Item {
        if generator {
            Item::Generator(elem)
        } else {
            Item::Microchip(elem)
        }
    }

    fn get_elem(&self) -> &String {
        match self {
            Item::Microchip(elem) | Item::Generator(elem) => elem,
        }
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Microchip(elem) => {
                write!(f, "{}M", elem.chars().next().unwrap().to_ascii_uppercase())
            }
            Self::Generator(elem) => {
                write!(f, "{}G", elem.chars().next().unwrap().to_ascii_uppercase())
            }
        }
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
            let solution = crate::day11::Puzzle {}
                .part_1(read_input(&FakeConfig::new(11, 1, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![];
        for case in cases {
            let solution = crate::day11::Puzzle {}
                .part_2(read_input(&FakeConfig::new(11, 2, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }
}
