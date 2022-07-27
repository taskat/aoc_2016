use std::{
    any::Any,
    collections::HashMap,
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::common;
use pathfinding;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let start = Area::new(input);
        let result = pathfinding::astar(
            &start,
            Area::neighbours,
            Area::heuristic_cost,
            Area::finished,
        );
        result.unwrap().1.to_string()
    }
    fn part_2(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let mut start = Area::new(input);
        start.pairs.push(CGPair{chip: 0, generator: 0, elem: String::from("elerium")});
        start.pairs.push(CGPair{chip: 0, generator: 0, elem: String::from("dilithium")});
        let result = pathfinding::astar(
            &start,
            Area::neighbours,
            Area::heuristic_cost,
            Area::finished,
        );
        result.unwrap().1.to_string()
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Area {
    pairs: Vec<CGPair>,
    current: usize,
}

impl Hash for Area {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pairs.hash(state);
        self.current.hash(state);
    }
}

impl Area {
    fn new(input: String) -> Area {
        let mut pairs: HashMap<String, CGPair> = HashMap::new();
        for (i, line) in input.split("\n").enumerate() {
            let mut words = line.split(' ').rev();
            while let Some(word) = words.next() {
                match word {
                    "generator" | "generator," | "generator." => {
                        let elem = words.next().unwrap().to_string();
                        pairs
                            .entry(elem.to_string())
                            .or_insert(CGPair::new(elem))
                            .generator = i;
                    }
                    "microchip" | "microchip," | "microchip." => {
                        let elem = words.next().unwrap().replace("-compatible", "").to_string();
                        pairs
                            .entry(elem.to_string())
                            .or_insert(CGPair::new(elem))
                            .chip = i;
                    }
                    _ => (),
                }
            }
        }
        let mut pairs = pairs.iter().map(|(_, p)| p.clone()).collect::<Vec<CGPair>>();
        pairs.sort();
        Area { pairs, current: 0 }
    }

    fn get_items_as_vec(&self, floor: usize) -> Vec<String> {
        self.pairs
            .iter()
            .map(|pair| pair.on_floor(floor))
            .flatten()
            .collect::<Vec<String>>()
    }

    fn neighbours(&self) -> Vec<(Area, usize)> {
        let items = self.get_items_as_vec(self.current);
        let mut neighbors: Vec<(Area, usize)> = vec![];
        let valid_items = Area::get_valid_items(items);
        for valid in &valid_items {
            let mut clone = self.clone();
            match self.current {
                0 => {
                    for v in valid {
                        clone.move_item(1, v);
                    }
                    clone.current = 1;
                }
                3 => {
                    for v in valid {
                        clone.move_item(2, v);
                    }
                    clone.current = 2
                }
                i => {
                    for v in valid {
                        clone.move_item(i - 1, v);
                    }
                    clone.current = i - 1;
                    neighbors.push((clone, 1));
                    clone = self.clone();
                    for v in valid {
                        clone.move_item(i + 1, v);
                    }
                    clone.current = i + 1;
                }
            }
            neighbors.push((clone, 1));
        }
        neighbors
            .iter()
            .filter(|(neighbour, _)| neighbour.is_valid())
            .map(|neighbour| neighbour.clone())
            .collect()
    }

    fn is_valid(&self) -> bool {
        for i in 0..4 {
            let items = self.get_items_as_vec(i);
            if items.iter().filter(|item| is_generator(item)).count() == 0 {
                continue;
            }
            if !items
                .iter()
                .filter(|item| is_chip(item))
                .all(|chip| has_generator(&items, &chip))
            {
                return false;
            }
        }
        true
    }

    fn move_item(&mut self, to: usize, item: &String) {
        let elem = &item[..item.len() - 2];
        let generator = is_generator(item);
        if generator {
            self.pairs
                .iter_mut()
                .find(|pair| pair.elem == elem)
                .unwrap()
                .generator = to;
        } else {
            self.pairs
                .iter_mut()
                .find(|pair| pair.elem == elem)
                .unwrap()
                .chip = to;
        }
        self.pairs.sort();
    }

    fn get_valid_items(items: Vec<String>) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = vec![];
        if items.len() == 0 {
            return result;
        }
        let generators = items.iter().filter(|item| is_generator(item)).count();
        //1 item taken
        for item in items.iter() {
            match item.chars().last().unwrap() {
                'M' => result.push(Vec::from([item.clone()])),
                'G' => {
                    if generators == 1 || !has_chip(&items, item) {
                        result.push(Vec::from([item.clone()]))
                    }
                }
                _ => panic!("Invalid item"),
            }
        }
        //2 items taken
        for i in 0..items.len() - 1 {
            for j in i + 1..items.len() {
                let item_1 = &items[i];
                let item_2 = &items[j];
                if is_chip(item_1) && is_chip(item_2) {
                    result.push(Vec::from([item_1.to_string(), item_2.to_string()]));
                }
                if (is_chip(item_1) && is_generator(item_2))
                    || (is_generator(item_1) && is_chip(item_2))
                {
                    if item_1[..item_1.len() - 1] == item_2[..item_2.len() - 1] {
                        result.push(Vec::from([item_1.to_string(), item_2.to_string()]));
                        continue;
                    }
                }
                if is_generator(item_1) && is_generator(item_2) {
                    if generators == 2 || (!has_chip(&items, item_1) && !has_chip(&items, item_2)) {
                        result.push(Vec::from([item_1.to_string(), item_2.to_string()]));
                    }
                }
            }
        }
        result
    }

    fn heuristic_cost(&self) -> usize {
        self.pairs
            .iter()
            .fold(0, |acc, p| acc + 3 - p.chip + 3 - p.generator)
    }

    fn finished(&self) -> bool {
        self.pairs.iter().all(|p| p.chip == 3 && p.generator == 3)
    }
}

fn is_chip(item: &String) -> bool {
    item.chars().last().unwrap() == 'M'
}

fn is_generator(item: &String) -> bool {
    item.chars().last().unwrap() == 'G'
}

fn has_chip(items: &Vec<String>, elem: &String) -> bool {
    items.contains(&(elem[..elem.len() - 1].to_string() + "M"))
}

fn has_generator(items: &Vec<String>, elem: &String) -> bool {
    items.contains(&(elem[..elem.len() - 1].to_string() + "G"))
}

#[derive(Eq, Ord, PartialOrd, Clone, Debug)]
struct CGPair {
    chip: usize,
    generator: usize,
    elem: String,
}

impl PartialEq for CGPair {
    fn eq(&self, other: &Self) -> bool {
        self.chip == other.chip && self.generator == other.generator
    }
}

impl Hash for CGPair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.chip.hash(state);
        self.generator.hash(state);
    }
}

impl CGPair {
    fn new(elem: String) -> CGPair {
        CGPair {
            chip: 0,
            generator: 0,
            elem,
        }
    }

    fn on_floor(&self, floor: usize) -> Vec<String> {
        let mut items = vec![];
        if floor == self.chip {
            items.push(self.elem.to_string() + " M");
        }
        if floor == self.generator {
            items.push(self.elem.to_string() + " G");
        }
        items
    }
}

#[cfg(test)]
mod tests {
    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases: Vec<(Data, &str)> = vec![(Data::Test(1), "11"), (Data::Real, "37")];
        for case in cases {
            let solution = crate::day11::Puzzle {}
                .part_1(read_input(&FakeConfig::new(11, 1, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    #[ignore]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Real, "61")
        ];
        for case in cases {
            let solution = crate::day11::Puzzle {}
                .part_2(read_input(&FakeConfig::new(11, 2, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }
}
