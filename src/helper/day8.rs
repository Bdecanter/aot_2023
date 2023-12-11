use regex::Regex;
use std::collections::HashMap;

use crate::helper::lcm;

#[derive(Debug, Clone)]
enum Direction {
    LEFT,
    RIGHT,
}
#[derive(Debug, Clone)]
struct Entrie {
    left_key: String,
    right_key: String,
}
#[derive(Debug)]
struct EntriesVector {
    step: i32,
    total_step: i32,
    key: String,
    direction_list: Vec<Direction>,
    entries_list: HashMap<String, Entrie>,
}
impl EntriesVector {
    fn next_step(&mut self) {
        let direction = &self.direction_list[self.step as usize];
        let actual_key = &self.entries_list.get(&self.key).unwrap();

        match direction {
            Direction::LEFT => {
                self.key = actual_key.left_key.to_string();
            }
            Direction::RIGHT => self.key = actual_key.right_key.to_string(),
        }

        if (self.step as usize) == self.direction_list.len() - 1 {
            self.step = 0;
        } else {
            self.step += 1;
        }
        self.total_step += 1;
    }
}

fn parse_direction(input: &str) -> Vec<Direction> {
    // RLRRLLLRRL - only R & L
    let first = input.split("\n").next();
    let mut direction_list = vec![];

    if first.is_none() {
        panic!("Dont find this fucking first line motherfucker...");
    }

    for char in first.unwrap().chars() {
        match char {
            'L' => direction_list.push(Direction::LEFT),
            'R' => direction_list.push(Direction::RIGHT),
            _ => {}
        }
    }

    return direction_list;
}

fn parse_entries(input: &str) -> HashMap<String, Entrie> {
    // let entrie_pattern = r"(?<key>[A-Z]{3}) = \((?<left>[A-Z]{3}), (?<right>[A-Z]{3})\)";
    let entrie_pattern = r"(?<key>[A-Z0-9]{3}) = \((?<left>[A-Z0-9]{3}), (?<right>[A-Z0-9]{3})\)";
    let my_regex = Regex::new(entrie_pattern);

    let mut entries_list: HashMap<String, Entrie> = HashMap::new();

    if my_regex.is_err() {
        panic!("Regex in entries is invalid motherfucker...");
    }

    for line in input.split("\n") {
        let my_match = my_regex.clone().unwrap().captures(line);

        if let Some(matches) = my_match {
            entries_list.insert(
                matches.name("key").unwrap().as_str().to_string(),
                Entrie {
                    left_key: matches.name("left").unwrap().as_str().to_string(),
                    right_key: matches.name("right").unwrap().as_str().to_string(),
                },
            );
        }
    }
    return entries_list;
}

fn get_keys_list(input: &str, char: char) -> Vec<String> {
    let mut entrie_pattern = r"(?<key>[A-Z]{1,2}xxx{1,2}) = \([A-Z]{3}, [A-Z]{3}\)";
    let rr = entrie_pattern.replace("xxx", char.to_string().as_str());

    let my_regex = Regex::new(&rr);

    let mut entries_list: Vec<String> = vec![];

    if my_regex.is_err() {
        panic!("Regex in entries is invalid motherfucker...");
    }

    for line in input.split("\n") {
        let my_match = my_regex.clone().unwrap().captures(line);

        if let Some(matches) = my_match {
            entries_list.push(matches.name("key").unwrap().as_str().to_string());
        }
    }
    return entries_list;
}

fn check_key(key: String) -> bool {
    let entrie_pattern = r"([A-Z]{2}Z)";
    let my_regex = Regex::new(entrie_pattern);

    if my_regex.is_err() {
        panic!("Regex in entries is invalid motherfucker...");
    }

    let is_match = my_regex.clone().unwrap().is_match(key.as_str());
    return is_match;
}

pub fn part1() {
    println!("--- day8 part1");

    let example_1 = include_str!("../files/day8/data.txt");
    let start_key = "AAA".to_string();
    let end_key = "ZZZ".to_string();

    let mut entries_vector = EntriesVector {
        step: 0,
        total_step: 0,
        key: start_key,
        entries_list: parse_entries(example_1),
        direction_list: parse_direction(example_1),
    };

    while entries_vector.key != end_key {
        println!("Actual key: {:?}", entries_vector.key);
        println!("Actual total: {:?}", entries_vector.total_step);
        entries_vector.next_step();
    }

    println!("\n🦀🦀🦀 TOTAL: {:?} 🦀🦀🦀", entries_vector.total_step);
}

pub fn part2() {
    println!("--- day8 partII");

    let data = include_str!("../files/day8/data.txt");

    let start_key_list = get_keys_list(data, 'A');

    println!("Start Keys: {:?}", start_key_list);

    let mut entries_list: Vec<EntriesVector> = vec![];

    for start_key in start_key_list {
        entries_list.push(EntriesVector {
            step: 0,
            total_step: 0,
            key: start_key,
            entries_list: parse_entries(data),
            direction_list: parse_direction(data),
        });
    }

    let mut total_step_vec = vec![];

    for mut entries in entries_list {
        while !check_key(entries.key.to_string()) {
            entries.next_step();
        }

        println!("Find in {:?} steps", entries.total_step);
        total_step_vec.push(entries.total_step as usize);
    }

    println!("\nValues find: {:?}\n", total_step_vec);

    let collected: &[usize] = total_step_vec.iter().as_slice();
    let res = lcm::lcm(collected);

    println!("🦀🦀🦀 LCM of this numbers is: {:?} 🦀🦀🦀", res);
    // LCM : 14321394058031
}
