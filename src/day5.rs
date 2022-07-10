use crate::common;
use md5;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, input: String) -> String {
        let mut pwd = String::new();
        let mut index = 0;
        while pwd.len() < 8 {
            if let Some((next, _)) = is_valid(format!("{}{}", input, index)) {
                pwd += &next.to_string();
                println!("{}", pwd);
            }
            index += 1;
        };
        pwd
    }
    fn part_2(&self, input: String) -> String {
        let mut pwd: Vec<String> = vec!["-".to_string(); 8];
        let mut index = 0;
        while pwd.contains(&"-".to_string()) {
            if let Some((pos, c)) = is_valid(format!("{}{}", input, index)) {
                if pos >= '0' && pos <= '7' {
                    let idx: usize = pos as usize - '0' as usize;
                    if pwd[idx] == "-" {
                        pwd[idx] = c.to_string();
                    }
                };
            }
            index += 1;
        };
        pwd.join("")
    }
}

fn is_valid(pwd: String) -> Option<(char, char)> {
    let hash = format!("{:x}", md5::compute(pwd));
    if hash[0..5] == *"00000" {
        return Some((hash.chars().nth(5).unwrap(), hash.chars().nth(6).unwrap()));
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    #[ignore]
    fn part_1() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Test(1), "18f47a30"),
            (Data::Real, "f77a0e6e"),
        ];
        for case in cases {
            let solution =
                crate::day5::Puzzle {}.part_1(read_input(&FakeConfig::new(5, 1, case.0)).unwrap());
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    #[ignore]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Test(1), "05ace8e3"),
            (Data::Real, "999828ec"),
        ];
        for case in cases {
            let solution =
                crate::day5::Puzzle {}.part_2(read_input(&FakeConfig::new(5, 2, case.0)).unwrap());
            assert_eq!(solution, case.1);
        }
    }
}