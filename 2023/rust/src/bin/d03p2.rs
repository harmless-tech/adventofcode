use std::fs;

fn main() {
    println!("Day 3 - Part 2");

    let str = fs::read_to_string("../_input/day3.txt").unwrap();
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

        let mut li = 0;
        while li < line.len() {
            let c = line[li];

            if c == '*' {
                let nums = {
                    let mut check_nums = [[false; 3]; 3];

                    let start_x = if li != 0 { li - 1 } else { li };
                    let start_y = if i != 0 { i - 1 } else { i };
                    let end_x = if li != line.len() - 1 { li + 1 } else { li };
                    let end_y = if i != lines.len() - 1 { i + 1 } else { i };

                    for (ix, x) in (start_x..=end_x).enumerate() {
                        for (iy, y) in (start_y..=end_y).enumerate() {
                            if lines[y][x].is_ascii_digit() {
                                check_nums[iy][ix] = true;
                            }
                        }
                    }

                    check_nums
                };

                let parts = {
                    let mut count: usize = 0;
                    let mut b = true;

                    for row in nums {
                        for item in row {
                            if item {
                                if b {
                                    count += 1;
                                }
                                b = false;
                            } else {
                                b = true;
                            }
                        }

                        b = true;
                    }

                    if !b {
                        count += 1;
                    }

                    count
                };

                let mut numbers_index = 0;
                let mut numbers = [String::new(), String::new()];
                if parts == 2 {
                    // Up
                    if nums[0][1] {
                        let s = &mut numbers[numbers_index];
                        s.push(lines[i - 1][li]);

                        let mut off = 1;
                        while li - off > 0 && lines[i - 1][li - off].is_ascii_digit() {
                            s.insert(0, lines[i - 1][li - off]);
                            off += 1;
                        }
                        if li - off == 0 && lines[i - 1][li - off].is_ascii_digit() {
                            s.insert(0, lines[i - 1][li - off]);
                        }

                        let mut off = 1;
                        while li + off < line.len() && lines[i - 1][li + off].is_ascii_digit() {
                            s.push(lines[i - 1][li + off]);
                            off += 1;
                        }

                        numbers_index += 1;
                    } else {
                        // Top Corners
                        // Top Left
                        if nums[0][0] {
                            let s = &mut numbers[numbers_index];

                            let mut off = 1;
                            while li - off > 0 && lines[i - 1][li - off].is_ascii_digit() {
                                s.insert(0, lines[i - 1][li - off]);
                                off += 1;
                            }
                            if li - off == 0 && lines[i - 1][li - off].is_ascii_digit() {
                                s.insert(0, lines[i - 1][li - off]);
                            }

                            numbers_index += 1;
                        }
                        // Top Right
                        if nums[0][2] {
                            let s = &mut numbers[numbers_index];

                            let mut off = 1;
                            while li + off < line.len() && lines[i - 1][li + off].is_ascii_digit() {
                                s.push(lines[i - 1][li + off]);
                                off += 1;
                            }

                            numbers_index += 1;
                        }
                    }
                    // Down
                    if nums[2][1] {
                        let s = &mut numbers[numbers_index];
                        s.push(lines[i + 1][li]);

                        let mut off = 1;
                        while li - off > 0 && lines[i + 1][li - off].is_ascii_digit() {
                            s.insert(0, lines[i + 1][li - off]);
                            off += 1;
                        }
                        if li - off == 0 && lines[i + 1][li - off].is_ascii_digit() {
                            s.insert(0, lines[i + 1][li - off]);
                        }

                        let mut off = 1;
                        while li + off < line.len() && lines[i + 1][li + off].is_ascii_digit() {
                            s.push(lines[i + 1][li + off]);
                            off += 1;
                        }

                        numbers_index += 1;
                    } else {
                        // Bottom Corners
                        // Bottom Left
                        if nums[2][0] {
                            let s = &mut numbers[numbers_index];

                            let mut off = 1;
                            while li - off > 0 && lines[i + 1][li - off].is_ascii_digit() {
                                s.insert(0, lines[i + 1][li - off]);
                                off += 1;
                            }
                            if li - off == 0 && lines[i + 1][li - off].is_ascii_digit() {
                                s.insert(0, lines[i + 1][li - off]);
                            }

                            numbers_index += 1;
                        }
                        // Bottom Right
                        if nums[2][2] {
                            let s = &mut numbers[numbers_index];

                            let mut off = 1;
                            while li + off < line.len() && lines[i + 1][li + off].is_ascii_digit() {
                                s.push(lines[i + 1][li + off]);
                                off += 1;
                            }

                            numbers_index += 1;
                        }
                    }
                    // Left
                    if nums[1][0] {
                        let s = &mut numbers[numbers_index];

                        let mut off = 1;
                        while li - off > 0 && lines[i][li - off].is_ascii_digit() {
                            s.insert(0, lines[i][li - off]);
                            off += 1;
                        }
                        if li - off == 0 && lines[i][li - off].is_ascii_digit() {
                            s.insert(0, lines[i][li - off]);
                        }

                        numbers_index += 1;
                    }
                    // Right
                    if nums[1][2] {
                        let s = &mut numbers[numbers_index];

                        let mut off = 1;
                        while li + off < line.len() && lines[i][li + off].is_ascii_digit() {
                            s.push(lines[i][li + off]);
                            off += 1;
                        }
                    }
                }

                total += str_to_u32(&numbers[0]) * str_to_u32(&numbers[1]);
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
            467835
        );
    }
}
