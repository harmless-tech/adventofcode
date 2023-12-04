use std::fs;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

struct Game {
    id: u32,
    games: Vec<SubSet>,
}

struct SubSet {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    println!("Day 2 - Part 1");

    let str = fs::read_to_string("../_input/day2.txt").unwrap();
    let lines = str.lines();

    let mut total = 0;
    for l in lines {
        total += process(l);
    }

    println!("Total: {}", total);
}

fn process(line: &str) -> u32 {
    let game = get_game(line);
    for sub in game.games.iter() {
        if sub.red > MAX_RED || sub.green > MAX_GREEN || sub.blue > MAX_BLUE {
            return 0;
        }
    }

    game.id
}

fn get_game(line: &str) -> Game {
    let colon = line.find(':').unwrap();
    let raw_id = &line[5..colon];
    let id = str_to_u32(raw_id);

    let raw_subsets = &line[(colon + 2)..line.len()];
    let raw_subsets: Vec<&str> = raw_subsets.split(';').map(|s| s.trim()).collect();
    let games: Vec<SubSet> = raw_subsets.iter().map(|s| get_subset(s)).collect();

    Game { id, games }
}

fn get_subset(sets: &str) -> SubSet {
    let subs: Vec<&str> = sets.split(',').map(|s| s.trim()).collect();

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for s in subs {
        let space = s.find(' ').unwrap();
        let num = str_to_u32(&s[0..space]);
        let color = &s[(space + 1)..s.len()];
        match color {
            "red" => red += num,
            "green" => green += num,
            "blue" => blue += num,
            _ => panic!("Missing color!"),
        }
    }

    SubSet { red, green, blue }
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
            process("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            1
        );
        assert_eq!(
            process("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            2
        );
        assert_eq!(
            process("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            0
        );
        assert_eq!(
            process("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            0
        );
        assert_eq!(
            process("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            5
        );
    }
}
