use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let v: Vec<&str> = input.trim().split("-").collect();
    let min: i32 = v[0].parse().unwrap();
    let max: i32 = v[1].parse().unwrap();

    part1(min, max);
    part2(min, max);
}

fn part1(min: i32, max: i32) {
    let mut valid_passwords = Vec::new();

    for i in min..max {
        if is_ascending(&i.to_string()) && contains_duplicate(&i.to_string()) {
            valid_passwords.push(i);
        }
    }

    println!("{}", valid_passwords.len());
}

fn part2(min: i32, max: i32) {
    let mut valid_passwords = Vec::new();

    for i in min..max {
        if is_ascending(&i.to_string()) && contains_duplicate_part2(&i.to_string()) {
            valid_passwords.push(i);
        }
    }

    println!("{}", valid_passwords.len());
}

fn is_ascending(s: &str) -> bool {
    let mut v: Vec<u32> = s
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    v.sort();
    let result: String = v.iter().map(ToString::to_string).collect();
    result == s
}

fn contains_duplicate(s: &str) -> bool {
    let mut counter = HashMap::new();

    for c in s.chars() {
        let count = counter.entry(c).or_insert(0);
        *count += 1;
    }

    counter.values().any(|&v| v >= 2)
}

fn contains_duplicate_part2(s: &str) -> bool {
    let mut counter = HashMap::new();

    for c in s.chars() {
        let count = counter.entry(c).or_insert(0);
        *count += 1;
    }

    counter.retain(|_, v| v == &2);
    counter.len() > 0
}
