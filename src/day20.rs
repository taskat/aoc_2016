use std::any::Any;

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, input: String, extra_param: Option<Box<dyn Any>>) -> String {
        let mut ranges = parse_input(input);
        let full_range = Range {
            from: 0,
            to: parse_extra(extra_param),
        };
        ranges = merge_ranges(ranges);
        println!("Merged ranges: {:?}", ranges.len());
        match first_non_blocked(&full_range, &ranges) {
            Ok(value) => return value.to_string(),
            Err(err) => err,
        }
    }
    fn part_2(&self, input: String, extra_param: Option<Box<dyn Any>>) -> String {
        let mut ranges = parse_input(input);
        ranges = merge_ranges(ranges);
        let all = parse_extra(extra_param);
        let blocked = ranges.iter().map(|r| r.len()).sum::<u32>();
        (all - blocked + 1).to_string()
    }
}

fn parse_extra(extra_param: Option<Box<dyn Any>>) -> u32 {
    match extra_param {
        Some(param) => str::parse::<u32>(&param.downcast::<String>().unwrap()).unwrap() as u32,
        None => 4294967295,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Range {
    from: u32,
    to: u32,
}

impl<T> From<T> for Range
where
    T: Into<String>,
{
    fn from(s: T) -> Self {
        let binding = s.into();
        let mut parts = binding.split('-');
        let from = parts.next().unwrap().parse::<u32>().unwrap();
        let to = parts.next().unwrap().parse::<u32>().unwrap();
        Range { from, to }
    }
}

impl Range {
    fn contains(&self, value: u32) -> bool {
        self.from <= value && self.to >= value
    }

    fn has_intersection(&self, other: &Range) -> bool {
        self.from <= other.to && self.to >= other.from
    }

    fn len(&self) -> u32 {
        self.to - self.from + 1
    }

    fn merge(&self, other: &Range) -> Result<Range, String> {
        if self.has_intersection(other) {
            Ok(Range {
                from: self.from.min(other.from),
                to: self.to.max(other.to),
            })
        } else {
            Err("No intersection".to_owned())
        }
    }
}

fn merge_ranges(ranges: Vec<Range>) -> Vec<Range> {
    let mut merged = Vec::new();
    let mut success = false;
    let mut used = vec![];
    for i in 0..ranges.len() {
        if used.contains(&i) {
            continue;
        }
        let mut intersected = false;
        for j in i+1..ranges.len() {
            if used.contains(&j) {
                continue;
            }
            let range = ranges.get(i).unwrap();
            let other = ranges.get(j).unwrap();
            if range.has_intersection(other) {
                success = true;
                intersected = true;
                let merged_range = range.merge(other).unwrap();
                merged.push(merged_range);
                used.push(i);
                used.push(j);
                break;
            }
        }
        if !intersected {
            merged.push(ranges.get(i).unwrap().clone());
        }
    }
    if success {
        return merge_ranges(merged);
    }
    return merged;
}   

fn first_non_blocked(range: &Range, ranges: &Vec<Range>) -> Result<u32, String> {
    let mut i = 0;
    while i < range.to {
        let mut contained = false;
        for r in ranges {
            if r.contains(i) {
                contained = true;
                i = r.to;
                break;
            }
        }
        if !contained {
            return Ok(i);
        }
        i += 1;
    }
    Err("No non-blocked".to_owned())
}

fn parse_input(input: String) -> Vec<Range> {
    input
        .lines()
        .map(|line| Range::from(line.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::any::Any;
    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Test(1), "3"),
            (Data::Real, "14975795"),
            ];
        for case in cases {
            let solution = crate::day20::Puzzle {}
                .part_1(read_input(&FakeConfig::new(20, 1, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str, Option<Box<dyn Any>>)> = vec![
            (Data::Test(1), "2", Some(Box::new(String::from("9")))),
            (Data::Real, "101", None),
        ];
        for case in cases {
            let solution = crate::day20::Puzzle {}
                .part_2(read_input(&FakeConfig::new(20, 2, case.0)).unwrap(), case.2);
            assert_eq!(solution, case.1);
        }
    }
}
