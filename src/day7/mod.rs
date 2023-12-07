use std::collections::HashSet;

use itertools::Itertools;

use crate::AOCInput;

#[derive(PartialEq, Eq, Debug)]
pub struct Hand {
    value: String,
    ty: HandType,
    bid: u32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HandType {
    Five = 6,
    Four = 5,
    Full = 4,
    Three = 3,
    Two = 2,
    One = 1,
    High = 0,
}

const CHARS: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

impl From<String> for HandType {
    fn from(value: String) -> Self {
        let counts = value.chars().counts();
        let values: HashSet<_> = counts.values().collect();

        let original = if counts.len() == 1 {
            Self::Five
        } else if values.contains(&4) && values.contains(&1) {
            Self::Four
        } else if values.contains(&3) && values.contains(&2) {
            Self::Full
        } else if values.contains(&3) {
            Self::Three
        } else if values.contains(&2) && counts.len() == 3 {
            Self::Two
        } else if values.contains(&2) {
            Self::One
        } else {
            Self::High
        };

        if let Some(n_js) = counts.get(&'J') {
            let without_jacks = HandType::from(
                value
                    .chars()
                    .enumerate()
                    .map(|(i, a)| if a == 'J' { i as u8 as char } else { a })
                    .collect::<String>(),
            );
            if *n_js > 0 {
                match without_jacks {
                    Self::High => match n_js {
                        1 => Self::One,
                        2 => Self::Three,
                        3 => Self::Four,
                        _ => Self::Five,
                    },
                    Self::One => match n_js {
                        1 => Self::Three,
                        2 => Self::Four,
                        _ => Self::Five,
                    },
                    Self::Two => Self::Full,
                    Self::Three => match n_js {
                        1 => Self::Four,
                        _ => Self::Five,
                    },
                    _ => Self::Five,
                }
            } else {
                without_jacks
            }
        } else {
            original
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.ty != other.ty {
            HandType::cmp(&self.ty, &other.ty)
        } else {
            for (a, b) in self.value.chars().zip(other.value.chars()) {
                let a = CHARS.iter().position(|t| *t == a).unwrap();
                let b = CHARS.iter().position(|t| *t == b).unwrap();
                match a.cmp(&b) {
                    std::cmp::Ordering::Greater => return std::cmp::Ordering::Less,
                    std::cmp::Ordering::Less => return std::cmp::Ordering::Greater,
                    _ => continue,
                }
            }
            std::cmp::Ordering::Greater
        }
    }
}

#[derive(Debug)]
struct Part1(u32);

impl From<AOCInput> for Part1 {
    fn from(value: AOCInput) -> Self {
        println!("WARNING: This is actually solving part 2");
        Self(
            value
                .0
                .lines()
                .map(|a| a.split_whitespace())
                .map(|mut split| {
                    let value = split.next().unwrap().to_string();
                    Hand {
                        value: value.clone(),
                        ty: value.into(),
                        bid: split.next().unwrap().parse::<u32>().unwrap(),
                    }
                })
                .sorted()
                .enumerate()
                .map(|(i, hand)| (i + 1) as u32 * hand.bid)
                .sum(),
        )
    }
}

#[derive(Debug)]
struct Part2(u32);

impl From<AOCInput> for Part2 {
    fn from(value: AOCInput) -> Self {
        Self(Part1::from(value).0)
    }
}

#[test]
fn tester() {
    let input = r#"32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483"#;

    println!("{:?}", Part1::from(AOCInput(input.to_string())));
    println!("{:?}", Part2::from(AOCInput(input.to_string())));
}

// 245773108
#[test]
fn part1() {
    let input = AOCInput::from("src/day7/input.txt");

    println!("{:?}", Part1::from(input));
}

#[test]
fn part2() {
    let input = AOCInput::from("src/day7/input.txt");

    println!("{:?}", Part2::from(input));
}
