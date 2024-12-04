use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let file = File::open("input.txt").expect("failed to open file");
    let mut reader = BufReader::new(file);

    let mut base_string = String::new();
    reader
        .read_to_string(&mut base_string)
        .expect("failed to read file contents");

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Invalid regex");
    let mut sum = 0;

    for cap in re.captures_iter(&base_string) {
        let x: usize = cap[1].parse::<usize>().expect("parse error");
        let y: usize = cap[2].parse::<usize>().expect("parse error");
        sum += x * y;
    }

    println!("{}", sum);
}
