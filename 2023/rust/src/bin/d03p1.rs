use std::fs;

fn main() {
    println!("Day 3 - Part 1");

    let str = fs::read_to_string("../_input/day03.txt").unwrap();
    let lines: Vec<&str> = str.lines().collect();

    let total = process(lines);
    println!("Total: {}", total);
}

fn process(lines: Vec<&str>) -> u32 {
    let mut l = Vec::new();
    for line in lines {
        l.push(line.chars().into_iter().collect::<Vec<char>>());
    }
    let lines = l;

    let mut total = 0;

    let mut i = 0;
    while i < lines.len() {
        let line = &lines[i];

        let mut num = String::new();
        let mut symbol = false;

        let mut li = 0;
        while li < line.len() {
            let c = line[li];

            if c.is_ascii_digit() {
                num.push(c);

                // Check
                if !symbol {
                    // Top Left
                    if i != 0
                        && li != 0
                        && !lines[i - 1][li - 1].is_ascii_digit()
                        && !lines[i - 1][li - 1].is_ascii_whitespace()
                        && !lines[i - 1][li - 1].eq(&'.')
                    {
                        symbol = true;
                    }
                    // Top Mid
                    if i != 0
                        && !lines[i - 1][li].is_ascii_digit()
                        && !lines[i - 1][li].is_ascii_whitespace()
                        && !lines[i - 1][li].eq(&'.')
                    {
                        symbol = true;
                    }
                    // Top Right
                    if i != 0
                        && li != line.len() - 1
                        && !lines[i - 1][li + 1].is_ascii_digit()
                        && !lines[i - 1][li + 1].is_ascii_whitespace()
                        && !lines[i - 1][li + 1].eq(&'.')
                    {
                        symbol = true;
                    }

                    // Mid Left
                    if li != 0
                        && !lines[i][li - 1].is_ascii_digit()
                        && !lines[i][li - 1].is_ascii_whitespace()
                        && !lines[i][li - 1].eq(&'.')
                    {
                        symbol = true;
                    }
                    // Mid Right
                    if li != line.len() - 1
                        && !lines[i][li + 1].is_ascii_digit()
                        && !lines[i][li + 1].is_ascii_whitespace()
                        && !lines[i][li + 1].eq(&'.')
                    {
                        symbol = true;
                    }

                    // Bottom Left
                    if i != lines.len() - 1
                        && li != 0
                        && !lines[i + 1][li - 1].is_ascii_digit()
                        && !lines[i + 1][li - 1].is_ascii_whitespace()
                        && !lines[i + 1][li - 1].eq(&'.')
                    {
                        symbol = true;
                    }
                    // Bottom Mid
                    if i != lines.len() - 1
                        && !lines[i + 1][li].is_ascii_digit()
                        && !lines[i + 1][li].is_ascii_whitespace()
                        && !lines[i + 1][li].eq(&'.')
                    {
                        symbol = true;
                    }
                    // Bottom Right
                    if i != lines.len() - 1
                        && li != line.len() - 1
                        && !lines[i + 1][li + 1].is_ascii_digit()
                        && !lines[i + 1][li + 1].is_ascii_whitespace()
                        && !lines[i + 1][li + 1].eq(&'.')
                    {
                        symbol = true;
                    }
                }
            }

            if !c.is_ascii_digit() || li + 1 >= line.len() {
                if symbol {
                    total += str_to_u32(&num);
                }

                num = String::new();
                symbol = false;
            }

            li += 1;
        }

        i += 1;
    }

    total
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
                r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
                    .lines()
                    .collect()
            ),
            4361
        );
    }
}
