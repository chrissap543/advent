use std::fs::File; 
use std::io::{prelude::*, BufReader}; 

pub fn solve_p1() {
    let mut count = 0; 

    let data = read_file("problems/day1part1"); 
    for i in 1..data.len() {
        if data[i] > data[i-1] {
            count += 1; 
        }
    }
    println!("{}", count); 
}

pub fn solve_p2() {
    let mut count = 0; 

    let data = read_file("problems/day1part2"); 
    let mut prev = data[0] + data[1] + data[2]; 
    for i in 1..data.len()-2 {
        let sum = data[i] + data[i+1] + data[i+2]; 
        if sum > prev {
            count += 1; 
        }
        prev = sum; 
    }
    println!("{}", count); 
}

fn read_file(file_name: &str) -> Vec<i32> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file); 

    let mut ret = Vec::new(); 
    for line in reader.lines() {
        ret.push(line.unwrap().parse::<i32>().unwrap()); 
    }

    ret
}
