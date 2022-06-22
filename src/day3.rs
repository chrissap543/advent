use crate::file_io;

pub fn solve_p1() {
    let lines = file_io::read_lines("problems/day3part1");
    let mut most_common = Vec::new();
    let mut least_common = Vec::new();

    for i in 0..lines[0].len() {
        let mut count_0 = 0;
        let mut count_1 = 0;
        for line in &lines {
            let c = line.as_bytes()[i] as char;
            match c {
                '0' => count_0 += 1,
                '1' => count_1 += 1,
                _ => panic!("line contains characters other than 0,1"),
            };
        }
        if count_1 > count_0 {
            most_common.push(1);
            least_common.push(0);
        } else {
            most_common.push(0);
            least_common.push(1);
        }
    }
    let gamma_str = most_common
        .into_iter()
        .map(|x| x.to_string())
        .collect::<String>();
    let epsilon_str = least_common
        .into_iter()
        .map(|x| x.to_string())
        .collect::<String>();
    let gamma = usize::from_str_radix(&gamma_str, 2).expect("Not a binary number");
    let epsilon = usize::from_str_radix(&epsilon_str, 2).expect("Not a binary number");
    println!("{}", gamma * epsilon);
}
