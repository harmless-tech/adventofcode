use std::{collections::HashMap, fs};

type Map<'a> = HashMap<&'a str, Node<'a>>;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}
impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unknown direction"),
        }
    }
}

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}
impl<'a> Node<'a> {
    fn from_str(s: &'a str) -> Result<Self, ()> {
        let (name, (left, right)) = s
            .split_once(" = ")
            .and_then(|s| {
                Some((
                    s.0,
                    s.1.strip_prefix('(')
                        .and_then(|s| s.strip_suffix(')'))
                        .and_then(|s| s.split_once(", "))?,
                ))
            })
            .ok_or(())?;

        Ok(Node { name, left, right })
    }
}

fn main() {
    println!("Day 8 - Part 2");

    let str = fs::read_to_string("../_input/day08.txt").unwrap();
    let lines: Vec<&str> = str.lines().collect();

    let total = process(lines);
    println!("Total: {}", total);
}

fn process(lines: Vec<&str>) -> u64 {
    let instructions: Vec<Direction> = lines[0].chars().map(|i| i.into()).collect();
    let (mut starting, map) = process_map(&lines[2..lines.len()]);

    let steps: Vec<u64> = starting
        .iter_mut()
        .map(|current_node| {
            let mut current_dir = 0;
            let mut hit = false;
            let mut counter = 0;

            while !hit {
                let node = map.get(current_node).unwrap();
                match instructions[current_dir] {
                    Direction::Left => *current_node = node.left,
                    Direction::Right => *current_node = node.right,
                }

                counter += 1;
                if current_node.ends_with('Z') {
                    hit = true;
                }

                current_dir += 1;
                if current_dir >= instructions.len() {
                    current_dir = 0;
                }
            }

            counter
        })
        .collect();

    steps.into_iter().reduce(lcm).unwrap()
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn process_map<'a>(lines: &'a [&'a str]) -> (Vec<&'a str>, Map<'a>) {
    let mut map = Map::new();
    let mut starting = Vec::new();

    for s in lines {
        let node = Node::from_str(s).unwrap();
        if node.name.ends_with('A') {
            starting.push(node.name);
        }
        map.insert(node.name, node);
    }

    (starting, map)
}

#[cfg(test)]
mod test {
    use crate::process;

    #[test]
    fn test() {
        assert_eq!(
            process(
                r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
                    .split('\n')
                    .collect()
            ),
            2
        );

        assert_eq!(
            process(
                r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
                    .split('\n')
                    .collect()
            ),
            6
        );
    }
}
