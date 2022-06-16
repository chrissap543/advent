use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

pub fn read_nums(file_name: &str) -> Vec<i32> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut ret = Vec::new();
    for line in reader.lines() {
        ret.push(line.unwrap().parse::<i32>().unwrap());
    }

    ret
}

pub fn read_lines_iter(file_name: &str) -> Lines<BufReader<File>> {
    let file = File::open(file_name).expect("could not find file");
    let reader = BufReader::new(file);
    reader.lines()
}

pub fn read_lines(file_name: &str) -> Vec<String> {
    read_lines_iter(file_name)
        .map(|s| s.expect("Could not parse line"))
        .collect()
}
