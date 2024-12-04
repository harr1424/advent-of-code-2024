use std::io::{BufRead, BufReader};
use std::fs::File;


fn main() {
    let file = File::open("input.txt").expect("unable to open file");
    let reader = BufReader::new(&file);

    let mut first: Vec<usize> = Vec::new();
    let mut second: Vec<usize> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("unable to read line)");
        let entry: Vec<&str> = line.split_whitespace().collect();
        first.push(entry[0].parse::<usize>().expect("unable to parse &str to usize"));
        second.push(entry[1].parse::<usize>().expect("unable to parse &str to usize"));
    }

    first.sort();
    second.sort();

    let mut diff: usize = 0;

    for i in 0..first.len() {
        diff += first[i].abs_diff(second[i]);
    }

    println!("Summed differences = {}", diff);
}
