use std::fs;

fn main() {
    println!("Day 01 - Part 2");

    let str = fs::read_to_string("../_input/day1.txt").unwrap();
    let lines = str.lines();

    let mut total = 0;
    for l in lines {
        total += get(l);
    }

    println!("Total: {}", total);
}

fn get(str: &str) -> u32 {
    let mut str = String::from(str);
    let mut i = 0_usize;
    while i < str.len() {
        let mut replace = ("0", 0);
        {
            let sub = &str[i..str.len()];
            if sub.starts_with("one") {
                replace = ("1", 2);
            } else if sub.starts_with("two") {
                replace = ("2", 2);
            } else if sub.starts_with("three") {
                replace = ("3", 4);
            } else if sub.starts_with("four") {
                replace = ("4", 3);
            } else if sub.starts_with("five") {
                replace = ("5", 3);
            } else if sub.starts_with("six") {
                replace = ("6", 2);
            } else if sub.starts_with("seven") {
                replace = ("7", 4);
            } else if sub.starts_with("eight") {
                replace = ("8", 4);
            } else if sub.starts_with("nine") {
                replace = ("9", 3);
            }
        }

        if replace.1 != 0 {
            str.replace_range(i..(i + replace.1), replace.0);
        }

        i += 1;
    }

    let str = str.as_str();

    let mut first = None;
    let mut last = None;

    for c in str.chars() {
        if c.is_ascii_digit() {
            if first.is_none() {
                first = c.to_digit(10);
            } else {
                last = c.to_digit(10);
            }
        }
    }

    if last.is_none() {
        last = first;
    }

    (first.unwrap() * 10) + last.unwrap()
}

#[cfg(test)]
mod test {
    use crate::get;

    #[test]
    fn test() {
        assert_eq!(get("two1nine"), 29);
        assert_eq!(get("eightwothree"), 83);
        assert_eq!(get("abcone2threexyz"), 13);
        assert_eq!(get("xtwone3four"), 24);
        assert_eq!(get("4nineeightseven2"), 42);
        assert_eq!(get("zoneight234"), 14);
        assert_eq!(get("7pqrstsixteen"), 76);

        assert_eq!(get("oneight"), 18);
    }
}
