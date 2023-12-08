use std::{collections::HashMap, fs};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Unknown card: {value}"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}
impl From<[Card; 5]> for HandType {
    fn from(value: [Card; 5]) -> Self {
        let mut map: HashMap<Card, usize> = HashMap::new();
        for c in value {
            *map.entry(c).or_default() += 1;
        }

        let length = map.keys().len();
        match length {
            1 => HandType::FiveKind,
            2 => {
                let vals: Vec<&usize> = map.values().collect();
                if *vals[0] == 4 || *vals[1] == 4 {
                    HandType::FourKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                let vals: Vec<&usize> = map.values().collect();
                if *vals[0] == 3 || *vals[1] == 3 || *vals[2] == 3 {
                    HandType::ThreeKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("Unknown hand type"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
    bet: u32,
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if !self.hand_type.eq(&other.hand_type) {
            return self.hand_type.cmp(&other.hand_type);
        } else {
            for (s, o) in self.cards.iter().zip(other.cards.iter()) {
                if !s.eq(o) {
                    return s.cmp(o);
                }
            }
        }

        std::cmp::Ordering::Equal
    }
}

fn main() {
    println!("Day 7 - Part 1");

    let str = fs::read_to_string("../_input/day07.txt").unwrap();
    let lines: Vec<&str> = str.lines().collect();

    let total = process(lines);
    println!("Total: {}", total);
}

fn process(lines: Vec<&str>) -> u32 {
    let mut hands: Vec<Hand> = lines.iter().map(|i| process_line(i)).collect();
    hands.sort();

    let mut counter = 0;
    for (i, h) in hands.iter().enumerate() {
        counter += h.bet * (i as u32 + 1);
    }

    counter
}

fn process_line(line: &str) -> Hand {
    let s: Vec<&str> = line.split_whitespace().collect();

    let mut cards: [Card; 5] = [Card::Two; 5];
    for (i, c) in s[0].chars().enumerate() {
        cards[i] = c.into();
    }

    Hand {
        cards,
        hand_type: cards.into(),
        bet: str_to_u32(s[1]),
    }
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
                r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
                    .split('\n')
                    .collect()
            ),
            6440
        );

        assert_eq!(
            process(
                r"2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41"
                    .split('\n')
                    .collect()
            ),
            6592
        );
    }
}
