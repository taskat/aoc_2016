use std::{any::Any, collections::HashMap};

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        get_most_occured(create_occurence_maps(input))
    }
    fn part_2(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        get_least_occured(create_occurence_maps(input))
    }
}

fn create_occurence_maps(input: String) -> Vec<HashMap<char, i32>> {
    let lines: Vec<&str> = input.split("\n").collect();
    let length = lines[0].len();
    let lines = lines.iter();
    let mut result: Vec<HashMap<char, i32>> = vec![HashMap::new(); length];
    lines.for_each(|line| {
        line.char_indices().for_each(|(i, c)| {
            let count = result[i].entry(c).or_insert(1);
            *count += 1;
        })
    });
    result
}

fn get_most_occured(occ: Vec<HashMap<char, i32>>) -> String {
    occ.iter()
        .map(|m| {
            m.iter()
                .max_by(|(_, a_occ), (_, b_occ)| a_occ.cmp(b_occ))
                .map(|(c, _)| c)
                .unwrap()
        })
        .collect()
}

fn get_least_occured(occ: Vec<HashMap<char, i32>>) -> String {
    occ.iter()
        .map(|m| {
            m.iter()
                .min_by(|(_, a_occ), (_, b_occ)| a_occ.cmp(b_occ))
                .map(|(c, _)| c)
                .unwrap()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases: Vec<(Data, &str)> = vec![(Data::Test(1), "easter"), (Data::Real, "xdkzukcf")];
        for case in cases {
            let solution = crate::day6::Puzzle {}
                .part_1(read_input(&FakeConfig::new(6, 1, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![(Data::Test(1), "advent"), (Data::Real, "cevsgyvd")];
        for case in cases {
            let solution = crate::day6::Puzzle {}
                .part_2(read_input(&FakeConfig::new(6, 2, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }
}
