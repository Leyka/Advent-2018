use std::fs;
use regex::Regex;
use chrono::{DateTime, Utc, Timelike};
use chrono::offset::TimeZone;
use std::collections::HashMap;

/*
    X récupérer dates et mettre dans une liste/dict
    X sort les dates
    Récupérer ID garde
    Calculer durée += (minutes) entre wakes up - falls asleep
    hash { id garde, value durée }
    retrieve max durée
*/

fn main() {
    let input = fs::read_to_string("input.txt").expect("Cannot read file");
    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> i64{
    let mut map_guard_totalsleep = HashMap::new();
    let mut map_guard_minfreq = HashMap::new();
    let re_guard_id = Regex::new(r"#(\d+)").unwrap();
    let organized_events = organize_events(input);

    let mut curr_guard = -1;
    let mut sleep_from = Utc::now();

    for org_event in organized_events {
        let (date, event) = &org_event;
        // Retrieve current guard id
        curr_guard = match &re_guard_id.captures(event) {
            Some (cap) => cap[1].parse().unwrap(),
            None => curr_guard
        };
        // Register slept time in minutes
        if event == "falls asleep" {
            sleep_from = date.to_owned();
        } else if event == "wakes up" {
            // Keep track of every minutes slept
            for min in sleep_from.minute()..date.minute() {
                let last_freq = map_guard_minfreq.get(&curr_guard).unwrap_or(&(min, 0)).1;
                map_guard_minfreq.insert(curr_guard, (min, last_freq + 1));
            }

            let last_duration = map_guard_totalsleep.get(&curr_guard).unwrap_or(&0);
            let duration = date.signed_duration_since(sleep_from).num_minutes();
            map_guard_totalsleep.insert(curr_guard, last_duration + duration);
        }
    }

    println!("{:?}", map_guard_totalsleep);
    println!("{:?}", map_guard_minfreq);
    let max_sleep_min = map_guard_totalsleep.values().max().unwrap();
    let guard = map_guard_totalsleep
                                    .iter()
                                    .find(|x| x.1 == max_sleep_min)
                                    .unwrap();
    println!("{:?}", guard);
    return guard.0 * guard.1;
}

fn organize_events(input: &str) -> Vec<(DateTime<Utc>, String)> {
    let mut events = Vec::new();
    let re = Regex::new(r"\[(.*)]\s(.*)").unwrap();
    for cap in re.captures_iter(input) {
        let (raw_date, raw_event) = (&cap[1], &cap[2]);
        let datetime = match Utc.datetime_from_str(raw_date, "%Y-%m-%d %H:%M") {
            Ok(dt) => dt,
            Err(_) => continue
        };

        events.push((datetime, raw_event.to_string()));
    }

    // Sort by date asc
    events.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    return events;
}