use std::fs;
use std::collections::{HashSet};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let number: i32 = line.parse().unwrap();
        sum += number;
    }

    sum
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;
    let mut set = HashSet::new();

    loop {
        for line in input.lines() {
            let change: i32 = line.parse().unwrap();
            sum += change;
            if set.contains(&sum) {
                return sum
            }
            set.insert(sum);
        }
    }
}