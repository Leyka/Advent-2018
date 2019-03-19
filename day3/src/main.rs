use std::fs;
use regex::{Regex, Captures};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Cannot read file");
    let mut state = [[0i32; 1000]; 1000];
    let re = Regex::new(r"#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)").unwrap();

    println!("Part 1: {}", part1(&input, &re, &mut state));
    println!("Part 2: {}", part2(&input, &re, &mut state));
}

fn part1(input: &str, re: &Regex, state: &mut[[i32;1000];1000]) -> i32 {
    let mut overlap_count = 0;
    for cap in re.captures_iter(input){
        let (id, x, y, width, height) = parse_capture(&cap);
        // Fill state
        for i in x..(x+width) {
            for j in y..(y+height) {
                if state[i][j] == 0 {
                    state[i][j] = id;
                } else if state[i][j] != -1 {
                    // Overlap, mark it (-1)
                    state[i][j] = -1;
                    overlap_count += 1;
                }
            }
        }
    }

    overlap_count
}

fn part2(input: &str, re: &Regex, state: &mut[[i32;1000];1000]) -> i32 {
    for cap in re.captures_iter(input) {
        let mut clean = true;
        let (id, x, y, width, height) = parse_capture(&cap);

        for i in x..(x+width) {
            for j in y..(y+height) {
                // Use the already mutated state from part 1 to see if
                // all squares are 100% filled with id
                if state[i][j] != id {
                    clean = false;
                }
            }

            if !clean {
                break;
            }
        }

        if clean {
            return id;
        }
    }

    -1
}

// Helper
fn parse_capture(cap: &Captures) -> (i32, usize, usize, usize, usize) {
   return (
       cap[1].parse().unwrap(),
       cap[2].parse().unwrap(),
       cap[3].parse().unwrap(),
       cap[4].parse().unwrap(),
       cap[5].parse().unwrap()
   )
}