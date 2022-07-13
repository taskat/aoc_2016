use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, input: String) -> String {
        input
            .split("\r\n")
            .filter(|line| support_tls(line))
            .count()
            .to_string()
    }
    fn part_2(&self, input: String) -> String {
        input
        .split("\r\n")
        .filter(|line| support_ssl(line))
        .count()
        .to_string()
    }
}

fn support_tls(line: &str) -> bool {
    let line = line.replace("[", "-?").replace("]", "-");
    let words = line.split("-");
    let mut has_abba = false;
    for word in words {
        if word.starts_with("?") {
            if !check_hypernet(&word[1..]) {
                return false;
            }
        } else {
            if contains_abba(word) {
                has_abba = true;
            }
        }
    };
    has_abba
}

fn check_hypernet(input: &str) -> bool {
    !contains_abba(input)
}

fn contains_abba(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    if chars.len() < 4 {
        return false;
    }
    for i in 0..=chars.len() - 4 {
        if is_abba(&chars[i..i + 4]) {
            return true;
        }
    }
    false
}

fn is_abba(input: &[char]) -> bool {
    input[0] == input[3] && input[1] == input[2] && input[0] != input[1]
}

fn support_ssl(line: &str) -> bool {
    let line = line.replace("[", "-?").replace("]", "-");
    let words = line.split("-");
    let mut supernet: Vec<&str> = vec![];
    let mut hypernet: Vec<&str> = vec![];
    words.for_each(|word| {
        if word.starts_with("?") {
            hypernet.push(&word[1..]);
        } else {
            supernet.push(word);
        }
    });
    let abas = get_abas(supernet);
    let real_babs = get_abas(hypernet);
    let expected_babs = convert_aba_to_bab(abas);
    real_babs.iter().any(|bab| {
        expected_babs.contains(bab)
    })
}

fn convert_aba_to_bab(abas: Vec<String>) -> Vec<String> {
    let mut babs = vec![String::new(); abas.len()];
    for (i, aba) in abas.iter().enumerate() {
        let aba = aba.to_string();
        let mut iter = aba.chars();
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        let bab = format!("{}{}{}", b, a, b);
        babs[i] = bab;
    }
    babs
}

fn get_abas(words: Vec<&str>) -> Vec<String> {
    let mut abas: Vec<String> = vec![];
    for word in words {
        let chars: Vec<char> = word.chars().collect();
        if chars.len() < 3 {
            continue;
        }
        for i in 0..=chars.len() - 3 {
            if is_aba(&chars[i..i + 3]) {
                abas.push(chars[i..i+3].iter().collect::<String>());
            }
        }
    }
    abas
}

fn is_aba(input: &[char]) -> bool {
    input[0] == input[2] && input[0] != input[1]
}

#[cfg(test)]
mod tests {
    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Test(1), "2"),
            (Data::Real, "118")
        ];
        for case in cases {
            let solution =
                crate::day7::Puzzle {}.part_1(read_input(&FakeConfig::new(7, 1, case.0)).unwrap());
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![
            (Data::Test(2), "3"),
            (Data::Real, "260")
        ];
        for case in cases {
            let solution =
                crate::day7::Puzzle {}.part_2(read_input(&FakeConfig::new(7, 2, case.0)).unwrap());
            assert_eq!(solution, case.1);
        }
    }
}
