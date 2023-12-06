use std::fs;

fn main() {
    println!("Day 6 - Part 1");

    let str = fs::read_to_string("../_input/day06.txt").unwrap();
    let lines: Vec<&str> = str.lines().collect();

    let total = process(lines);
    println!("Total: {}", total);
}

fn process(lines: Vec<&str>) -> u32 {
    let times = process_line(lines[0]);
    let distance = process_line(lines[1]);

    let mut total = 1;
    for (t, d) in times.iter().zip(distance.iter()) {
        let mut counter = 0;
        for speed in 0..=*t {
            let time_left = t - speed;
            let dis = time_left * speed;
            if dis > *d {
                counter += 1;
            }
        }
        total *= counter;
    }

    total
}

fn process_line(line: &str) -> Vec<u32> {
    line.split(':').collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(str_to_u32)
        .collect()
}

fn str_to_u32(str: &str) -> u32 {
    let mut acc = 0;
    for (i, c) in str.chars().enumerate() {
        acc += (10_u32.pow((str.len() - i - 1) as u32)) * c.to_digit(10).unwrap();
    }
    acc
}

#[cfg(test)]
mod test {
    use crate::process;

    #[test]
    fn test() {
        assert_eq!(
            process(
                r"Time:      7  15   30
Distance:  9  40  200"
                    .split('\n')
                    .collect()
            ),
            288
        );
    }
}
