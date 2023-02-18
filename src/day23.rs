use std::{any::Any, collections::HashMap};

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, _input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let mut commands = parse_commands(_input);
        optimize(&mut commands);
        let mut computer = Computer::new();
        let mut i = 0;
        while i >= 0 && i < commands.len() as i32 {
            let (offset, toggle) = commands[i as usize].execute(&mut computer, &commands, i);
            i += offset;
            if let Some((i, command)) = toggle {
                commands[i as usize] = command;
            }
        }
        computer.registers.get("a").unwrap().to_string()
    }
    fn part_2(&self, _input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let mut commands = parse_commands(_input);
        optimize(&mut commands);
        let mut computer = Computer::new();
        computer.registers.insert("a".to_owned(), 12);
        let mut i = 0;
        while i >= 0 && i < commands.len() as i32 {
            let (offset, toggle) = commands[i as usize].execute(&mut computer, &commands, i);
            i += offset;
            if let Some((i, command)) = toggle {
                commands[i as usize] = command;
            }
        }
        computer.registers.get("a").unwrap().to_string()
    }
}

#[derive(Debug)]
struct Computer {
    registers: HashMap<String, i32>,
}

impl Computer {
    fn new() -> Computer {
        let mut registers = HashMap::new();
        for i in 'a'..='d' {
            registers.insert(i.to_string(), 0);
        }
        registers.insert("a".to_owned(), 7);
        Computer { registers }
    }

    fn get_value(&self, value: &str) -> i32 {
        match value.parse::<i32>() {
            Ok(v) => v,
            Err(_) => *self.registers.get(value).unwrap(),
        }
    }
}

fn parse_commands(input: String) -> Vec<Box<dyn Command>> {
    input
        .split("\n")
        .map(|command| parse_command(command.to_owned()))
        .collect::<Vec<Box<dyn Command>>>()
}

fn parse_command(command: String) -> Box<dyn Command> {
    let mut words = command.split(" ");
    match words.next().unwrap() {
        "cpy" => Box::new(Cpy::new(
            words.next().unwrap().to_owned(),
            words.next().unwrap().to_owned(),
        )),
        "inc" => Box::new(Inc::new(words.next().unwrap().to_owned())),
        "dec" => Box::new(Dec::new(words.next().unwrap().to_owned())),
        "jnz" => Box::new(Jnz::new(
            words.next().unwrap().to_owned(),
            words.next().unwrap().to_owned(),
        )),
        "tgl" => Box::new(Tgl::new(words.next().unwrap().to_owned())),
        _ => panic!("Unknown command"),
    }
}

trait Command: std::fmt::Debug {
    fn execute(
        &self,
        computer: &mut Computer,
        commands: &Vec<Box<dyn Command>>,
        i: i32,
    ) -> (i32, Option<(i32, Box<dyn Command>)>);
    fn toggle(&self) -> Box<dyn Command>;
    fn is_toggle(&self) -> bool {
        false
    }
    fn loop_length(&self) -> i32 {
        0
    }
    fn loop_repeat(&self) -> String {
        "".to_owned()
    }
    fn loop_variable(&self) -> String {
        "".to_owned()
    }
    fn is_loop_variable_used(&self, _variable: String) -> bool {
        false
    }
    fn simple_optimizable(&self) -> bool {
        false
    }
    fn simple_optimized(&self, _rep_amount: String) -> Box<dyn Command> {
        panic!("Not optimizable");
    }
}

#[derive(Debug)]
struct Cpy {
    from: String,
    to: String,
}

impl Command for Cpy {
    fn execute(
        &self,
        computer: &mut Computer,
        _commands: &Vec<Box<dyn Command>>,
        _i: i32,
    ) -> (i32, Option<(i32, Box<dyn Command>)>) {
        let value = computer.get_value(&self.from);
        computer
            .registers
            .entry(self.to.to_owned())
            .and_modify(|v| *v = value);
        (1, None)
    }
    fn toggle(&self) -> Box<dyn Command> {
        Box::new(Jnz {
            x: self.from.to_owned(),
            y: self.to.to_owned(),
        })
    }
    fn simple_optimizable(&self) -> bool {
        true
    }
    fn simple_optimized(&self, _rep_amount: String) -> Box<dyn Command> {
        Box::new(Cpy::new(self.from.clone(), self.to.clone()))
    }
}

impl Cpy {
    fn new(from: String, to: String) -> Cpy {
        Cpy { from, to }
    }
}

#[derive(Debug)]
struct Jnz {
    x: String,
    y: String,
}

impl Command for Jnz {
    fn execute(
        &self,
        computer: &mut Computer,
        _commands: &Vec<Box<dyn Command>>,
        _i: i32,
    ) -> (i32, Option<(i32, Box<dyn Command>)>) {
        let x = computer.get_value(&self.x);
        if x != 0 {
            return (computer.get_value(&self.y), None);
        }
        (1, None)
    }
    fn toggle(&self) -> Box<dyn Command> {
        Box::new(Cpy {
            from: self.x.to_owned(),
            to: self.y.to_owned(),
        })
    }
    fn loop_length(&self) -> i32 {
        -self.y.parse::<i32>().unwrap()
    }
    fn loop_repeat(&self) -> String {
        self.x.clone()
    }
    fn loop_variable(&self) -> String {
        self.x.clone()
    }
    fn simple_optimized(&self, _rep_amount: String) -> Box<dyn Command> {
        Box::new(Cpy::new("0".to_owned(), self.x.to_owned()))
    }
}

impl Jnz {
    fn new(x: String, y: String) -> Jnz {
        Jnz { x, y }
    }
}

#[derive(Debug)]
struct Inc {
    x: String,
}

impl Command for Inc {
    fn execute(
        &self,
        computer: &mut Computer,
        _commands: &Vec<Box<dyn Command>>,
        _i: i32,
    ) -> (i32, Option<(i32, Box<dyn Command>)>) {
        computer
            .registers
            .entry(self.x.to_owned())
            .and_modify(|v| *v += 1);
        (1, None)
    }
    fn toggle(&self) -> Box<dyn Command> {
        Box::new(Dec {
            x: self.x.to_owned(),
        })
    }
    fn simple_optimizable(&self) -> bool {
        true
    }
    fn is_loop_variable_used(&self, variable: String) -> bool {
        variable == self.x
    }
    fn simple_optimized(&self, rep_amount: String) -> Box<dyn Command> {
        Box::new(Add::new(rep_amount, self.x.to_owned(), true))
    }
}

impl Inc {
    fn new(x: String) -> Inc {
        Inc { x }
    }
}

#[derive(Debug)]
struct Dec {
    x: String,
}

impl Command for Dec {
    fn execute(
        &self,
        computer: &mut Computer,
        _commands: &Vec<Box<dyn Command>>,
        _i: i32,
    ) -> (i32, Option<(i32, Box<dyn Command>)>) {
        computer
            .registers
            .entry(self.x.to_owned())
            .and_modify(|v| *v -= 1);
        (1, None)
    }
    fn toggle(&self) -> Box<dyn Command> {
        Box::new(Inc {
            x: self.x.to_owned(),
        })
    }
    fn simple_optimizable(&self) -> bool {
        true
    }
    fn simple_optimized(&self, rep_amount: String) -> Box<dyn Command> {
        Box::new(Add::new(rep_amount, self.x.to_owned(), false))
    }
    fn is_loop_variable_used(&self, variable: String) -> bool {
        variable == self.x
    }
}

impl Dec {
    fn new(x: String) -> Dec {
        Dec { x }
    }
}

#[derive(Debug)]
struct Tgl {
    x: String,
}

impl Command for Tgl {
    fn execute(
        &self,
        computer: &mut Computer,
        commands: &Vec<Box<dyn Command>>,
        i: i32,
    ) -> (i32, Option<(i32, Box<dyn Command>)>) {
        let x = computer.get_value(&self.x);
        if i + x >= 0 && i + x < commands.len() as i32 {
            let new_command = commands[i as usize + x as usize].toggle();
            return (1, Some((i + x, new_command)));
        }
        (1, None)
    }
    fn toggle(&self) -> Box<dyn Command> {
        Box::new(Inc {
            x: self.x.to_owned(),
        })
    }
    fn is_toggle(&self) -> bool {
        true
    }
}

impl Tgl {
    fn new(x: String) -> Tgl {
        Tgl { x }
    }
}

#[derive(Debug)]
struct Nop {}

impl Command for Nop {
    fn execute(
        &self,
        _computer: &mut Computer,
        _commands: &Vec<Box<dyn Command>>,
        _i: i32,
    ) -> (i32, Option<(i32, Box<dyn Command>)>) {
        (1, None)
    }
    fn toggle(&self) -> Box<dyn Command> {
        Box::new(Nop {})
    }
    fn simple_optimizable(&self) -> bool {
        true
    }
    fn simple_optimized(&self, _rep_amount: String) -> Box<dyn Command> {
        Box::new(Nop {})
    }
}

#[derive(Debug)]
struct Add {
    amount: String,
    to: String,
    positive: bool,
}

impl Command for Add {
    fn execute(
        &self,
        computer: &mut Computer,
        _commands: &Vec<Box<dyn Command>>,
        _i: i32,
    ) -> (i32, Option<(i32, Box<dyn Command>)>) {
        let amount = computer.get_value(&self.amount);
        let to = computer.get_value(&self.to);
        if self.positive {
            computer
                .registers
                .entry(self.to.to_owned())
                .and_modify(|v| *v = to + amount);
        } else {
            computer
                .registers
                .entry(self.to.to_owned())
                .and_modify(|v| *v = to - amount);
        }
        (1, None)
    }

    fn toggle(&self) -> Box<dyn Command> {
        Box::new(Nop {})
    }
    fn simple_optimizable(&self) -> bool {
        true
    }
    fn simple_optimized(&self, rep_amount: String) -> Box<dyn Command> {
        Box::new(Mul::new(
            rep_amount,
            self.to.to_owned(),
            self.amount.to_owned(),
            self.positive,
        ))
    }
}

impl Add {
    fn new(amount: String, to: String, positive: bool) -> Add {
        Add {
            amount,
            to,
            positive,
        }
    }
}

#[derive(Debug)]
struct Mul {
    amount: String,
    to: String,
    additive: String,
    positive: bool,
}

impl Command for Mul {
    fn execute(
        &self,
        computer: &mut Computer,
        _commands: &Vec<Box<dyn Command>>,
        _i: i32,
    ) -> (i32, Option<(i32, Box<dyn Command>)>) {
        let to = computer.get_value(&self.to);
        let additive = computer.get_value(&self.additive);
        let amount = computer.get_value(&self.amount);
        let prod = if self.positive {
            additive * amount.abs()
        } else {
            -1 * additive * amount.abs()
        };
        computer
            .registers
            .entry(self.to.to_owned())
            .and_modify(|v| *v = to + prod);
        (1, None)
    }
    fn toggle(&self) -> Box<dyn Command> {
        Box::new(Nop {})
    }
}

impl Mul {
    fn new(amount: String, to: String, additive: String, positive: bool) -> Mul {
        Mul {
            amount,
            to,
            additive,
            positive,
        }
    }
}

fn optimize(commands: &mut Vec<Box<dyn Command>>) {
    let mut i = 0;
    while i < commands.len() {
        if commands[i].is_toggle() {
            return;
        }
        let loop_length = commands[i].loop_length();
        let loop_repeat = commands[i].loop_repeat();
        let loop_variable = commands[i].loop_variable();
        if loop_length > 0 {
            let mut simple_optimizable = true;
            for j in i - loop_length as usize..i {
                if !commands[j].simple_optimizable() {
                    simple_optimizable = false;
                    break;
                }
            }
            if simple_optimizable {
                for j in i - loop_length as usize..=i {
                    if commands[j].is_loop_variable_used(loop_variable.clone()) {
                        commands[j] = Box::new(Nop {});
                        continue;
                    }
                    commands[j] = commands[j].simple_optimized(loop_repeat.clone());
                }
            }
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases: Vec<(Data, &str)> = vec![(Data::Test(1), "3"), (Data::Real, "10880")];
        for case in cases {
            let solution = crate::day23::Puzzle {}
                .part_1(read_input(&FakeConfig::new(23, 1, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Real, "479007440")
        ];
        for case in cases {
            let solution = crate::day23::Puzzle {}
                .part_2(read_input(&FakeConfig::new(23, 2, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }
}
