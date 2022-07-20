use std::{collections::HashMap, fmt::Debug};

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, input: String) -> String {
        let mut factory = Factory::new(input);
        factory.work([61, 17]).unwrap().to_string()
    }
    fn part_2(&self, input: String) -> String {
        let mut factory = Factory::new(input);
        factory.work([-1, -1]);
        let mut product = 1;
        for i in 0..3 {
            product *= factory.get_output(&i).chips[0];
        }
        product.to_string()
    }
}

#[derive(Debug)]
struct Factory {
    robots: HashMap<i32, Bot>,
    outputs: HashMap<i32, Output>,
}

impl Factory {
    fn new(input: String) -> Factory {
        let lines = input.split("\r\n");
        let mut f = Factory {
            outputs: HashMap::new(),
            robots: HashMap::new(),
        };
        for line in lines {
            if line.starts_with("value") {
                let mut numbers = line
                    .split(" ")
                    .map(|word| word.parse::<i32>())
                    .filter(|result| result.is_ok())
                    .map(|result| result.unwrap());
                let chip = numbers.next().unwrap();
                let bot_number = numbers.next().unwrap();
                f.add_chip_to_bot(chip, bot_number);
            } else {
                f.parse_command(line);
            };
        }
        f
    }

    fn work(&mut self, search: [i32; 2]) -> Option<i32> {
        while let Ok(bot_number) = self.find_next() {
            let result = self.get_bot(&bot_number).work().unwrap();
            if search.contains(&result[0].chip) && search.contains(&result[1].chip) {
                return Some(bot_number);
            }
            for i in 0..2 {
                let giving = &result[i];
                if giving.is_bot {
                    self.get_bot(&giving.to).add_chip(giving.chip);
                } else {
                    self.get_output(&giving.to).add_chip(giving.chip);
                }
            }
        }
        None
    }

    fn find_next(&self) -> Result<i32, String> {
        for bot in self.robots.values() {
            if bot.can_work() {
                return Ok(bot.number);
            }
        }
        Err(String::from("No next move"))
    }

    fn add_chip_to_bot(&mut self, chip: i32, bot_number: i32) {
        let bot = self
            .robots
            .entry(bot_number)
            .or_insert(Bot::new(&bot_number));
        bot.add_chip(chip);
    }

    fn parse_command(&mut self, line: &str) {
        let words = line.split(" ");
        let mut numbers: Vec<i32> = vec![];
        let mut keywords: Vec<&str> = vec![];
        for word in words {
            if let Ok(i) = word.parse::<i32>() {
                numbers.push(i);
            }
            if word == "bot" || word == "output" {
                keywords.push(word);
            }
        }
        let bot_number = numbers[0];
        let low_number = numbers[1];
        let high_number = numbers[2];
        let bot = self.get_bot(&bot_number);
        bot.set_low(low_number, keywords[1] == "bot");
        bot.set_high(high_number, keywords[2] == "bot");
        if keywords[1] == "output" {
            self.get_output(&low_number);
        }
        if keywords[2] == "output" {
            self.get_output(&high_number);
        }
    }

    fn get_bot(&mut self, number: &i32) -> &mut Bot {
        self.robots.entry(*number).or_insert(Bot::new(number))
    }

    fn get_output(&mut self, number: &i32) -> &mut Output {
        self.outputs.entry(*number).or_insert(Output::new(number))
    }
}

trait Bin {
    fn add_chip(&mut self, chip: i32);
    fn get_number(&self) -> i32;
}

impl Debug for dyn Bin {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Bin{{{:?}}}", self)
    }
}

#[derive(Debug)]
struct Output {
    number: i32,
    chips: Vec<i32>,
}

impl Output {
    fn new(number: &i32) -> Output {
        Output {
            chips: vec![],
            number: *number,
        }
    }
}

impl Bin for Output {
    fn add_chip(&mut self, chip: i32) {
        self.chips.push(chip);
    }
    fn get_number(&self) -> i32 {
        self.number
    }
}

struct Bot {
    number: i32,
    chips: Vec<i32>,
    low: i32,
    low_bot: bool,
    high: i32,
    high_bot: bool,
}

impl Debug for Bot {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let low = match self.low {
            -1 => String::from("None"),
            i => i.to_string(),
        };
        let high = match self.high {
            -1 => String::from("None"),
            i => i.to_string(),
        };
        let low_target = if self.low_bot { "bot" } else { "output" };
        let high_target = if self.high_bot { "bot" } else { "output" };
        write!(
            f,
            "Bot{{{}, chips: {:?},low: {} {}, high: {} {}}}",
            self.number, self.chips, low_target, low, high_target, high
        )
    }
}

#[derive(Clone, Copy)]
struct Giving {
    to: i32,
    is_bot: bool,
    chip: i32,
}

impl Bot {
    fn new(number: &i32) -> Bot {
        Bot {
            number: *number,
            chips: vec![],
            low: -1,
            low_bot: false,
            high: -1,
            high_bot: false,
        }
    }

    fn set_low(&mut self, low: i32, is_bot: bool) {
        self.low = low;
        self.low_bot = is_bot;
    }

    fn set_high(&mut self, high: i32, is_bot: bool) {
        self.high = high;
        self.high_bot = is_bot;
    }

    fn can_work(&self) -> bool {
        self.chips.len() ==  2
    }

    fn work(&mut self) -> Result<[Giving; 2], String> {
        if self.chips.len() != 2 {
            return Err(String::from("There is not enough chips"));
        }
        let mut result = [Giving{to: -1, is_bot: false, chip: -1}; 2];
        if self.chips[0] < self.chips[1] {
            result[0] = Giving {
                to: self.low,
                is_bot: self.low_bot,
                chip: self.chips[0],
            };
            result[1] = Giving {
                to: self.high,
                is_bot: self.high_bot,
                chip: self.chips[1],
            };
        } else {
            result[0] = Giving {
                to: self.low,
                is_bot: self.low_bot,
                chip: self.chips[1],
            };
            result[1] = Giving {
                to: self.high,
                is_bot: self.high_bot,
                chip: self.chips[0],
            };
        }
        self.chips.clear();
        Ok(result)
    }
}

impl Bin for Bot {
    fn add_chip(&mut self, chip: i32) {
        self.chips.push(chip);
    }
    fn get_number(&self) -> i32 {
        self.number
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
            let solution = crate::day10::Puzzle {}
                .part_1(read_input(&FakeConfig::new(10, 1, case.0)).unwrap());
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![];
        for case in cases {
            let solution = crate::day10::Puzzle {}
                .part_2(read_input(&FakeConfig::new(10, 2, case.0)).unwrap());
            assert_eq!(solution, case.1);
        }
    }
}
