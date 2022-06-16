use crate::file_io;

pub fn solve_p1() {
    let mut count = 0;

    let data = file_io::read_nums("problems/day1part1");
    for i in 1..data.len() {
        if data[i] > data[i - 1] {
            count += 1;
        }
    }
    println!("{}", count);
}

pub fn solve_p2() {
    let mut count = 0;

    let data = file_io::read_nums("problems/day1part2");
    let mut prev = data[0] + data[1] + data[2];
    for i in 1..data.len() - 2 {
        let sum = data[i] + data[i + 1] + data[i + 2];
        if sum > prev {
            count += 1;
        }
        prev = sum;
    }
    println!("{}", count);
}
