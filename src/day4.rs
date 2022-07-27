use std::{any::Any, collections::HashMap};

use crate::common;

pub struct Puzzle {}

impl common::Puzzle for Puzzle {
    fn part_1(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let rooms = create_rooms(input);
        format!(
            "{}",
            rooms
                .iter()
                .filter(|room| !room.is_decoy())
                .map(|room| room.id)
                .sum::<i32>()
        )
    }

    fn part_2(&self, input: String, _extra_param: Option<Box<dyn Any>>) -> String {
        let mut rooms = create_rooms(input);
        rooms = rooms
            .iter()
            .filter(|room| !room.is_decoy())
            .map(|room: &Room| -> Room { room.decrypt() })
            .filter(|room| room.real_name.contains("north"))
            .collect::<Vec<Room>>();
        if let Some(room) = rooms.get(0) {
            return room.id.to_string();
        }
        String::from("Not found")
    }
}

fn create_rooms(input: String) -> Vec<Room> {
    input.split("\n").map(|line| Room::new(line)).collect()
}

#[derive(Debug)]
struct Room {
    encrypted_name: String,
    real_name: String,
    id: i32,
    checksum: String,
}

impl Room {
    fn new(mut line: &str) -> Room {
        line = &line[..line.len() - 1];
        let parts: Vec<&str> = line.split("[").collect();
        let checksum = parts[1];
        let parts: Vec<&str> = parts[0].split("-").collect();
        let id: i32 = parts.last().unwrap().parse().unwrap();
        let name = parts[..parts.len() - 1].join("-");
        Room {
            encrypted_name: name,
            real_name: String::new(),
            id: id,
            checksum: checksum.to_string(),
        }
    }
    fn is_decoy(&self) -> bool {
        let mut letters: HashMap<char, i32> = HashMap::new();
        for c in self.encrypted_name.chars() {
            let count = letters.entry(c).or_insert(1);
            *count += 1;
        }
        let mut count_vec = letters
            .iter()
            .filter(|pair| pair.0 != &'-')
            .collect::<Vec<(&char, &i32)>>();
        count_vec.sort_by(|a, b| {
            if a.1 == b.1 {
                return a.0.cmp(b.0);
            }
            b.1.cmp(a.1)
        });
        let checksum: String = count_vec[0..5].iter().map(|pair| pair.0).collect();
        checksum != self.checksum
    }

    fn decrypt(&self) -> Room {
        let diff = 'z' as u8 - 'a' as u8 + 1;
        let number_of_rotations = (self.id % diff as i32) as u8;
        let mut name = String::new();
        for c in self.encrypted_name.chars() {
            match c {
                '-' => name += " ",
                c => {
                    let mut c = (c as u8) + number_of_rotations;
                    if c > 'z' as u8 {
                        c -= diff;
                    }
                    name += &(c as char).to_string();
                }
            }
        }
        Room {
            real_name: name,
            encrypted_name: self.encrypted_name.clone(),
            id: self.id,
            checksum: self.checksum.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::common_test::FakeConfig;
    use crate::common::{read_input, Data, Puzzle};

    #[test]
    fn part_1() {
        let cases: Vec<(Data, &str)> = vec![(Data::Test(1), "1514"), (Data::Real, "409147")];
        for case in cases {
            let solution = crate::day4::Puzzle {}
                .part_1(read_input(&FakeConfig::new(4, 1, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }

    #[test]
    fn part_2() {
        let cases: Vec<(Data, &str)> = vec![(Data::Test(2), "Not found"), (Data::Real, "991")];
        for case in cases {
            let solution = crate::day4::Puzzle {}
                .part_2(read_input(&FakeConfig::new(4, 2, case.0)).unwrap(), None);
            assert_eq!(solution, case.1);
        }
    }
}
