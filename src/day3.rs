use std::any::Any;

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let triangles = create_triangles_by_row(input);
        let solution = count_valid(triangles);
        format! {"{}", solution}
    }
    fn part_2(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let triangles = create_triangles_by_col(input);
        let solution = count_valid(triangles);
        format! {"{}", solution}
    }
}

fn count_valid(triangles: Vec<Triangle>) -> usize {
    triangles
        .iter()
        .filter(|triangle| triangle.is_valid())
        .count()
}

#[derive(Debug)]
struct Triangle(i32, i32, i32);

impl Triangle {
    fn from(input: &str) -> Triangle {
        let mut parts = input
            .split("  ")
            .map(|part| part.trim())
            .filter(|part| part != &"")
            .map(|part| part.parse::<i32>().unwrap());
        Triangle(
            parts.next().unwrap(),
            parts.next().unwrap(),
            parts.next().unwrap(),
        )
    }

    fn is_valid(&self) -> bool {
        self.0 + self.1 > self.2 && self.0 + self.2 > self.1 && self.1 + self.2 > self.0
    }
}

fn create_triangles_by_row(input: String) -> Vec<Triangle> {
    input
        .split("\r\n")
        .map(|line| Triangle::from(line))
        .collect()
}

fn break_into_parts(line: Option<&str>) -> Vec<i32> {
    line.unwrap()
        .split("  ")
        .map(|part| part.trim())
        .filter(|part| part != &"")
        .map(|part| part.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn create_triangles_by_col(input: String) -> Vec<Triangle> {
    let mut lines = input.split("\r\n");
    let mut triangles = vec![];
    while let Some(first) = lines.next() {
        let second = lines.next();
        let third = lines.next();
        let first = break_into_parts(Some(first));
        let second = break_into_parts(second);
        let third = break_into_parts(third);
        for i in 0..3 {
            triangles.push(Triangle(first[i], second[i], third[i]));
        }
    }
    triangles
}

#[cfg(test)]
mod tests {
    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Test(1), "0"),
            (Data::Test(2), "3"),
            (Data::Real, "1050"),
        ];
        for case in cases {
            let solution = crate::day3::Puzzle {}
                .part_1(read_input(&FakeConfig::new(3, 1, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![(Data::Real, "1921")];
        for case in cases {
            let solution = crate::day3::Puzzle {}
                .part_2(read_input(&FakeConfig::new(3, 2, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }
}
