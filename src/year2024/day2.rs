use std::slice::Iter;
use std::iter::Peekable;

pub fn part1(input: String) -> String {
    let mut safe_levels_count = 0;
    for report in input.lines() {
        let mut report = report.split_whitespace().peekable();
        let mut safe = true;
        let mut last_level = report.next().unwrap().parse::<u8>().unwrap();
        let increasing = last_level < report.peek().unwrap().parse::<u8>().unwrap();
        for current_level in report {
            let current_level = current_level.parse::<u8>().unwrap();
            if last_level == current_level {
                safe = false;
                break;
            }
            else if last_level > current_level && (increasing == true || last_level - current_level > 3) {
                safe = false;
                break;
            }
            else if last_level < current_level && (increasing == false || current_level - last_level > 3) {
                safe = false;
                break;
            }
            last_level = current_level;
        }
        if safe {
            safe_levels_count += 1;
        }
    }
    format!("{}", safe_levels_count)
}

pub fn part2(input: String) -> String {
    let mut safe_levels_count = 0;
    for report in input.lines() {
        let report = report
            .split_whitespace()
            .map(|level| level
            .parse::<u8>()
            .unwrap())
            .collect::<Vec<u8>>();
        let mut level_configurations = Vec::new();
        level_configurations.push(report
            .clone()
            .iter()
            .peekable()
        );
        let mut safe = false;
        if is_safe(report.clone().iter().peekable()) {
            safe = true;
        }
        for index in 0..report.len() {
            let mut current_level_configuration = report.clone();
            current_level_configuration.remove(index);
            if is_safe(current_level_configuration.iter().peekable()) {
                safe = true;
            }
        }
        if safe == true {
            safe_levels_count += 1;
        }
    }
    format!("{}", safe_levels_count)
}

fn is_safe(mut report: Peekable<Iter<u8>>) -> bool {
    let mut last_level = report.next().unwrap();
    let increasing = last_level < report.peek().unwrap();
    for current_level in report {
        let current_level = current_level;
        if last_level == current_level {
            return false;
        }
        else if last_level > current_level && (increasing == true || last_level - current_level > 3) {
            return false;
        }
        else if last_level < current_level && (increasing == false || current_level - last_level > 3) {
            return false;
        }
        last_level = current_level;
    }
    true
}
