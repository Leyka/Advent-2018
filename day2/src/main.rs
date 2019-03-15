use std::fs;
use std::collections::{HashMap};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Cannot read file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut letters = HashMap::new();
    let (mut two_freq, mut three_freq) = (0, 0);

    for line in input.lines() {
        // Count letter freq in each line
        for char in line.chars() {
            let freq = *letters.get(&char).unwrap_or(&0);
            letters.insert(char, freq+1);
        }

        // 2 letters freq ?
        let has_two = letters.values().any(|&freq| freq == 2);
        if has_two {
            two_freq += 1;
        }
        // 3 letters freq ?
        let has_three = letters.values().any(|&freq| freq == 3);
        if has_three {
            three_freq += 1;
        }

        letters.clear();
    }

    two_freq * three_freq
}

fn part2(input: &str) -> String {
    let mut common = String::new();
    for (i, line) in input.lines().enumerate() {
        for (j, next_line) in input.lines().enumerate() {
            if i == j {
                continue;
            }

            common = match find_common_letters(line, next_line) {
                Ok(s) => s,
                Err(_) => continue
            };

            // Found common, return
            return common;
        }
    }

    return common;
}

fn find_common_letters(a: &str, b: &str) -> Result<String, &'static str> {
    let mut common = String::new();
    let mut count_diff = 0;
    for (i, _) in a.chars().enumerate() {
        if a.chars().nth(i).unwrap() == b.chars().nth(i).unwrap()  {
            common += &a.chars().nth(i).unwrap().to_string();
        } else {
            count_diff += 1;
            if count_diff > 1 {
                return Err("Not common");
            }
        }
    }

    Ok(common)
}
