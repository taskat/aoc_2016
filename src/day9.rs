use std::{any::Any, str::Chars};

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        decompress_once(input).len().to_string()
    }
    fn part_2(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        decompress_full(input).to_string()
    }
}

fn decompress_once(input: String) -> String {
    let mut result = String::new();
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        match c {
            '(' => {
                let marker = get_marker(&mut chars);
                result += &marker.decompress_once(&mut chars);
            }
            c => result.push(c),
        }
    }
    result
}

fn decompress_full(input: String) -> usize {
    let mut len = 0;
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        match c {
            '(' => {
                let marker = get_marker(&mut chars);
                len += &marker.decompress_full(&mut chars);
            }
            _ => len += 1,
        }
    }
    len
}

fn get_marker(input: &mut Chars<'_>) -> Marker {
    let mut marker = String::new();
    while let Some(c) = input.next() {
        match c {
            ')' => return Marker::new(marker),
            c => marker.push(c),
        };
    }
    panic!("Marker not finished!");
}

#[derive(Debug)]
struct Marker {
    length: usize,
    repeat: usize,
}

impl Marker {
    fn new(input: String) -> Marker {
        let mut parts = input.split('x');
        let length = parts.next().unwrap().parse::<usize>().unwrap();
        let repeat = parts.next().unwrap().parse::<usize>().unwrap();
        Marker { length, repeat }
    }

    fn decompress_once(&self, chars: &mut Chars<'_>) -> String {
        let mut part = String::new();
        for _ in 0..self.length {
            part.push(chars.next().unwrap());
        }
        part.repeat(self.repeat)
    }

    fn decompress_full(&self, chars: &mut Chars<'_>) -> usize {
        let mut part = String::new();
        for _ in 0..self.length {
            part.push(chars.next().unwrap());
        }
        self.repeat * decompress_full(part)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Test(1), "6"),
            (Data::Test(2), "7"),
            (Data::Test(3), "9"),
            (Data::Test(4), "11"),
            (Data::Test(5), "6"),
            (Data::Test(6), "18"),
            (Data::Real, "107035"),
        ];
        for case in cases {
            let solution = crate::day9::Puzzle {}
                .part_1(read_input(&FakeConfig::new(9, 1, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Test(3), "9"),
            (Data::Test(6), "20"),
            (Data::Test(7), "241920"),
            (Data::Test(8), "445"),
            (Data::Real, "11451628995"),
        ];
        for case in cases {
            let solution = crate::day9::Puzzle {}
                .part_2(read_input(&FakeConfig::new(9, 2, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }
}
