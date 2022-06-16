use crate::file_io::read_lines;

pub fn solve_p1() {
    let lines = read_lines("problems/day2");

    let mut depth = 0;
    let mut horiz = 0;
    for s in lines {
        let mut split = s.split_whitespace();

        let direction = split.next().expect("No direction");
        let distance = split
            .next()
            .expect("No distance")
            .parse::<i32>()
            .expect("Distance not a number");
        if direction == "forward" {
            horiz += distance;
        } else if direction == "down" {
            depth += distance;
        } else if direction == "up" {
            depth -= distance;
        } else {
            panic!("Direction is incorrect");
        }
    }
    println!("{}", depth * horiz);
}

pub fn solve_p2() {
    let lines = read_lines("problems/day2");

    let mut depth = 0;
    let mut horiz = 0;
    let mut aim = 0;

    for s in lines {
        let mut split = s.split_whitespace();

        let direction = split.next().expect("Could not parse direction");
        let distance = split
            .next()
            .expect("Could not parse distance")
            .parse::<i32>()
            .expect("Distance not a number");
        match direction {
            "forward" => {
                horiz += distance;
                depth += aim * distance;
            }
            "down" => aim += distance,
            "up" => aim -= distance,
            _ => panic!("Direction is incorrect"),
        };
    }
    println!("{}", depth * horiz);
}
