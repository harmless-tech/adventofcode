use std::fs;

fn main() {
    println!("Day 01 - Part 1");

    let str = fs::read_to_string("../_input/day1.txt").unwrap();
    let lines = str.lines();

    let mut total = 0;
    for l in lines {
        total += get(l);
    }

    println!("Total: {}", total);
}

fn get(str: &str) -> u32 {
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
        assert_eq!(get("1abc2"), 12);
        assert_eq!(get("pqr3stu8vwx"), 38);
        assert_eq!(get("a1b2c3d4e5f"), 15);
        assert_eq!(get("treb7uchet"), 77);
    }
}
