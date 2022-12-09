use std::any::Any;

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, _input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let number_of_elves = get_number_of_elves(&_input);
        let mut elves = create_elves(number_of_elves);
        let winner = find_winner(&mut elves);
        winner.number.to_string()
    }
    fn part_2(&self, _input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let number_of_elves = get_number_of_elves(&_input);
        let mut elves = create_elves(number_of_elves);
        let winner = find_winner_2(&mut elves);
        winner.number.to_string()
    }
}

fn get_number_of_elves(input: &str) -> usize {
    input.parse().unwrap()
}

#[derive(Clone, Copy)]
struct Elf {
    number: usize,
    out: bool,
}

impl Elf {
    fn new(number: usize) -> Elf {
        Elf { number, out: false }
    }
}

fn create_elves(number_of_elves: usize) -> Vec<Elf> {
    let mut elves = Vec::with_capacity(number_of_elves);
    for i in 0..number_of_elves {
        elves.push(Elf::new(i + 1));
    }
    elves
}

fn find_winner(elves: &mut Vec<Elf>) -> Elf {
    let mut i = 0;
    loop {
        if elves[i].out {
            i = (i + 1) % elves.len();
            continue;
        }
        let mut stolen = false;
        for j in 1..elves.len() {
            let next_idx = (j + i) % elves.len();
            if !elves[next_idx].out {
                elves[next_idx].out = true;
                stolen = true;
                break;
            }
        }
        if !stolen {
            break;
        }
        i = (i + 1) % elves.len();
    }
    for elf in elves {
        if !elf.out {
            return *elf;
        }
    };
    panic!("No winner found");
}

fn find_winner_2(elves: &mut Vec<Elf>) -> Elf {
    let mut remaining = elves.len();
    let mut idx = remaining / 2;
    if remaining % 2 == 0 {
        while remaining > 1 {
            elves[idx].out = true;
            remaining -= 1;
            if remaining == 1 {
                break;
            }
            idx = get_next(elves, idx);
            elves[idx].out = true;
            remaining -= 1;
            if remaining == 1 {
                break;
            }
            idx = get_next(elves, idx);
            idx = get_next(elves, idx);
        }
    } else {
        while remaining > 1 {
            elves[idx].out = true;
            remaining -= 1;
            if remaining == 1 {
                break;
            }
            idx = get_next(elves, idx);
            idx = get_next(elves, idx);
            elves[idx].out = true;
            remaining -= 1;
            if remaining == 1 {
                break;
            }
            idx = get_next(elves, idx);
        }
    }
    idx = get_next(elves, idx);
    elves[idx] 
}

fn get_next(elves: &Vec<Elf>, current: usize) -> usize {
    let mut idx = (current + 1) % elves.len();
    while elves[idx].out {
        idx = (idx + 1) % elves.len();
    }
    idx
}

#[cfg(test)]
mod tests {
    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Test(1), "3"),
            (Data::Test(2), "5"),
            (Data::Real, "1830117")
        ];
        for case in cases {
            let solution = crate::day19::Puzzle {}
                .part_1(read_input(&FakeConfig::new(19, 1, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Test(1), "2"),
            (Data::Test(2), "3"),
            (Data::Real, "1417887")
        ];
        for case in cases {
            let solution = crate::day19::Puzzle {}
                .part_2(read_input(&FakeConfig::new(19, 2, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }
}
