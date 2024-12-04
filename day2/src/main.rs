use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("unable to open input file");
    let reader = BufReader::new(file);
    let mut safe_lines = 0;

    for line in reader.lines() {
        if is_safe(&line.expect("faild to read line")) {
            safe_lines += 1;
        }
    }

    println!("{}", safe_lines);
}

fn is_safe(line: &str) -> bool {
    let reports: Vec<usize> = line
        .split_whitespace()
        .map(|r| r.parse::<usize>().expect("parse error"))
        .collect();

    if is_ascending_within_limit(reports.clone()) || is_descending_within_limit(reports) {
        return true;
    }

    false
}

fn is_ascending_within_limit(mut series: Vec<usize>) -> bool {
    let mut last_seen: Option<usize> = None;

    series.reverse();
    while let Some(report) = series.pop() {
        if last_seen == None {
            last_seen = Some(report);
            continue;
        }

        if last_seen.unwrap() >= report || last_seen.unwrap().abs_diff(report) > 3 {
            return false;
        }

        last_seen = Some(report);
    }

    true
}

fn is_descending_within_limit(mut series: Vec<usize>) -> bool {
    let mut last_seen: Option<usize> = None;

    while let Some(report) = series.pop() {
        if last_seen == None {
            last_seen = Some(report);
            continue;
        }

        if last_seen.unwrap() >= report || last_seen.unwrap().abs_diff(report) > 3 {
            return false;
        }

        last_seen = Some(report);
    }

    true
}
