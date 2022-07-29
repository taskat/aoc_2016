use std::{any::Any, char};

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        find_nth_key_index(64, input, 1).to_string()
    }
    fn part_2(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        find_nth_key_index(64, input, 2017).to_string()
    }
}

fn get_hash(s: String) -> String {
    format!("{:x}", md5::compute(s))
}

fn find_nth_key_index(n: i32, base: String, hashings: i32) -> i32 {
    let mut index = 0;
    let mut keys: Vec<Key> = vec![];
    let mut possible_keys: Vec<Key> = vec![];
    while keys.len() < n as usize || possible_keys.len() != 0 {
        let mut hash = get_hash(format!("{}{}", base, index));
        for _ in 0..hashings - 1 {
            hash = get_hash(hash);
        }
        let mut to_remove: Vec<usize> = vec![];
        for (i, key) in possible_keys.iter().enumerate() {
            if key.index + 1000 < index {
                to_remove.push(i);
            } else {
                if key.contains_five(&hash) {
                    to_remove.push(i);
                    keys.push(key.clone());
                }
            }
        }
        for i in to_remove.iter().rev() {
            possible_keys.swap_remove(*i);
        }
        if keys.len() < n as usize {
            if let Some(c) = get_key_char(hash) {
                possible_keys.push(Key::new(index, c));
            }
        }
        
        index += 1;
    }
    keys.sort();
    keys.get((n - 1) as usize).unwrap().index
}

fn get_key_char(hash: String) -> Option<char> {
    let chars = hash.chars().collect::<Vec<char>>();
    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 1] && chars[i] == chars[i + 2] {
            return Some(chars[i]);
        }
    }
    None
}

#[derive(Clone, Eq, Debug)]
struct Key {
    index: i32,
    c: char
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Key {
    fn new(index: i32, c: char) -> Key {
        Key { index, c }
    }

    fn contains_five(&self, hash: &String) -> bool {
        let fives = self.c.to_string().repeat(5);
        hash.contains(&fives)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Test(1), "22728"),
            (Data::Real, "15168")
        ];
        for case in cases {
            let solution = crate::day14::Puzzle {}
                .part_1(read_input(&FakeConfig::new(14, 1, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    #[ignore]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Test(1), "22859"),
            (Data::Real, "20864")
        ];
        for case in cases {
            let solution = crate::day14::Puzzle {}
                .part_2(read_input(&FakeConfig::new(14, 2, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }
}
