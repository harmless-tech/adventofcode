use std::fs;

#[derive(Debug)]
enum Maps {
    Seeds(Vec<i64>),
    To(Vec<Mappings>),
}

#[derive(Debug)]
struct Mappings {
    dest_start: i64,
    src_start: i64,
    range_len: i64,
}

fn main() {
    println!("Day 5 - Part 1");

    let str = fs::read_to_string("../_input/day05.txt").unwrap();

    let total = process(&str);
    println!("Total: {}", total);
}

fn process(string: &str) -> i64 {
    let mut input = Vec::new();
    for line in string.lines() {
        input.push(line);
    }
    let input = input;

    // Load items
    let mut maps = Vec::new();
    let mut store = Vec::new();

    let mut i = 0;
    while i < input.len() {
        let l = input.get(i).unwrap();
        if l.is_empty() || i + 1 == input.len() {
            maps.push(convert_items(&store));
            store = Vec::new();
        } else {
            store.push(l);
        }

        i += 1;
    }

    // Do mappings
    let mut vals = match maps.remove(0) {
        Maps::Seeds(s) => s,
        _ => panic!("Seeds not at 0!"),
    };
    for m in maps {
        match m {
            Maps::To(s) => map_vals(&mut vals, s),
            _ => panic!("Not at 0!"),
        }
    }

    vals.iter().fold(i64::MAX, |acc, x| acc.min(*x))
}

fn map_vals(vals: &mut Vec<i64>, mappings: Vec<Mappings>) {
    let mut changes = vec![false; vals.len()];
    for m in mappings {
        let dest_start = m.dest_start;
        let src_start = m.src_start;
        let src_end = src_start + m.range_len - 1;
        let diff = dest_start - src_start;

        for (i, v) in vals.iter_mut().enumerate() {
            if *v >= src_start && *v <= src_end && !changes[i] {
                *v += diff;
                changes[i] = true;
            }
        }
    }
}

fn convert_items(lines: &[&str]) -> Maps {
    let first = lines.first().unwrap();
    if first.starts_with("seeds:") {
        let mut seeds = first.split(' ').collect::<Vec<&str>>();
        seeds.remove(0);
        Maps::Seeds(seeds.iter().map(|i| str_to_i64(i)).collect())
    } else {
        Maps::To(make_mappings(&lines[1..lines.len()]))
    }
}

fn make_mappings(lines: &[&str]) -> Vec<Mappings> {
    lines
        .iter()
        .map(|i| {
            let vals: Vec<&str> = i.split(' ').collect();
            let vals: Vec<i64> = vals.iter().map(|i| str_to_i64(i)).collect();
            Mappings {
                dest_start: vals[0],
                src_start: vals[1],
                range_len: vals[2],
            }
        })
        .collect()
}

fn str_to_i64(str: &str) -> i64 {
    let mut acc = 0;
    for (i, c) in str.chars().enumerate() {
        acc += (10_i64.pow((str.len() - i - 1) as u32)) * c.to_digit(10).unwrap() as i64;
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
                r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"
            ),
            35
        );
    }
}
