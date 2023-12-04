use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::AOCInput;

#[derive(Debug)]
struct Part1(u32);

impl From<AOCInput> for Part1 {
    fn from(value: AOCInput) -> Self {
        Self(
            value
                .0
                .lines()
                .map(|line| {
                    let (winning, mine) = line.split_once(":").unwrap().1.split_once("|").unwrap();

                    let winning: HashSet<u32> = winning
                        .split_whitespace()
                        .map(|a| a.parse().unwrap())
                        .collect();
                    let mine: HashSet<u32> = mine
                        .split_whitespace()
                        .map(|a| a.parse().unwrap())
                        .collect();

                    let count = winning.intersection(&mine).collect_vec().len();
                    if count >= 1 {
                        2_u32.pow(count.saturating_sub(1) as u32)
                    } else {
                        0
                    }
                })
                .sum(),
        )
    }
}

#[derive(Debug)]
struct Part2(u32);

impl From<AOCInput> for Part2 {
    fn from(value: AOCInput) -> Self {
        let mut cards: HashMap<u32, u32> = Default::default();

        for (i, line) in value.0.lines().enumerate() {
            let (winning, mine) = line.split_once(":").unwrap().1.split_once("|").unwrap();

            let winning: HashSet<u32> = winning
                .split_whitespace()
                .map(|a| a.parse().unwrap())
                .collect();
            let mine: HashSet<u32> = mine
                .split_whitespace()
                .map(|a| a.parse().unwrap())
                .collect();

            let count = winning.intersection(&mine).collect_vec().len();

            let card_num = cards.get(&(i as u32)).cloned().unwrap_or_default() + 1;
            for j in (i + 1)..(i + 1 + count) {
                *cards.entry(j as u32).or_default() += card_num;
            }

            *cards.entry(i as u32).or_default() += 1;
        }

        Self(cards.values().sum())
    }
}

#[test]
fn tester() {
    let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    println!("{:?}", Part1::from(AOCInput(input.to_string())));
    println!("{:?}", Part2::from(AOCInput(input.to_string())));
}

#[test]
fn part1() {
    let input = AOCInput::from("src/day4/input.txt");

    println!("{:?}", Part1::from(input));
}

#[test]
fn part2() {
    let input = AOCInput::from("src/day4/input.txt");

    println!("{:?}", Part2::from(input));
}
