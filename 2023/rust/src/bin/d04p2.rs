use std::fs;

fn main() {
    println!("Day 4 - Part 2");

    let str = fs::read_to_string("../_input/day04.txt").unwrap();

    let total = process(&str);
    println!("Total: {}", total);
}

fn process(string: &str) -> u32 {
    let mut cards = Vec::new();
    for line in string.lines() {
        cards.push(process_card(line));
    }
    let cards = cards;

    let mut count = 0;
    for i in 0..cards.len() {
        count += process_num(i, &cards);
    }

    count
}

fn process_num(card: usize, cards: &[u32]) -> u32 {
    let mut count = 1;
    let c = cards[card] as usize;
    for i in 1..=c {
        if card + i < cards.len() {
            count += process_num(card + i, cards);
        }
    }
    count
}

fn process_card(line: &str) -> u32 {
    let nums: Vec<&str> = line.split(':').collect::<Vec<&str>>()[1]
        .split('|')
        .collect();
    let winning_nums: Vec<u32> = nums[0]
        .split(' ')
        .filter_map(|i| {
            let s = i.trim();
            if !s.is_empty() {
                Some(str_to_u32(s))
            } else {
                None
            }
        })
        .collect();
    let have_nums: Vec<u32> = nums[1]
        .split(' ')
        .filter_map(|i| {
            let s = i.trim();
            if !s.is_empty() {
                Some(str_to_u32(s))
            } else {
                None
            }
        })
        .collect();

    let mut count = 0;
    for n in have_nums {
        if winning_nums.contains(&n) {
            count += 1;
        }
    }

    count
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
                r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            30
        );
    }
}
