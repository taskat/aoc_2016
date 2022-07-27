use std::{any::Any, collections::HashMap};

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let commands = create_commands(input);
        let mut computer = Computer::new();
        let mut i = 0;
        while i < commands.len() {
            i = (i as i32 + computer.execute(&commands[i])) as usize
        }
        computer.registers["a"].to_string()
    }
    fn part_2(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let commands = create_commands(input);
        let mut computer = Computer::new();
        computer.registers.entry("c".to_owned()).and_modify(|v| *v = 1);
        let mut i = 0;
        while i < commands.len() {
            i = (i as i32 + computer.execute(&commands[i])) as usize
        }
        computer.registers["a"].to_string()
    }
}

fn create_commands(input: String) -> Vec<String> {
    input.split("\n").map(|command| command.to_owned()).collect::<Vec<String>>()
}

struct Computer {
    registers: HashMap<String, i32>
}

impl Computer {
    fn new() -> Computer {
        let mut registers = HashMap::new();
        for i in 'a'..='d' {
            registers.insert(i.to_string(), 0);
        }
        Computer { registers }
    }

    fn execute(&mut self, command: &String) -> i32{
        let mut words = command.split(" ");
        match words.next().unwrap() {
            "cpy" => {
                let from = words.next().unwrap();
                let value = self.get_value(from);
                let to = words.next().unwrap();
                self.registers.entry(to.to_owned()).and_modify(|v| *v = value);
            },
            "inc" => {self.registers.entry(words.next().unwrap().to_owned()).and_modify(|v| *v += 1);},
            "dec"  => {self.registers.entry(words.next().unwrap().to_owned()).and_modify(|v| *v -= 1);},
            "jnz" => {
                if self.get_value(words.next().unwrap()) != 0 {
                    return words.next().unwrap().parse::<i32>().unwrap();
                }
            }
            _ => panic!("Invalid command")
        };
        1
    }

    fn get_value(&self, data: &str) -> i32 {
        match data.parse::<i32>() {
            Ok(i) => i,
            Err(_) => self.registers[data],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Test(1), "42"),
            (Data::Real, "318077")
        ];
        for case in cases {
            let solution = crate::day12::Puzzle {}
                .part_1(read_input(&FakeConfig::new(12, 1, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    #[ignore]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Test(1), "42"),
            (Data::Real, "9227731")
        ];
        for case in cases {
            let solution = crate::day12::Puzzle {}
                .part_2(read_input(&FakeConfig::new(12, 2, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }
}
