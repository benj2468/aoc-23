use std::collections::{HashMap, HashSet};

use crate::AOCInput;

#[derive(Debug)]
pub struct Day3 {
    symbols: HashSet<(usize, usize)>,
    numbers: HashMap<(usize, usize), Vec<u32>>,
    potential_gears: HashSet<(usize, usize)>,
}

fn surrounding_values((x, y): (usize, usize), l: usize) -> HashSet<(usize, usize)> {
    (0..l)
        .into_iter()
        .flat_map(|k| {
            let (x, y) = (x as isize, (y + k) as isize);
            [
                (x - 1, y - 1),
                (x - 1, y),
                (x - 1, y + 1),
                (x, y - 1),
                (x, y + 1),
                (x + 1, y - 1),
                (x + 1, y),
                (x + 1, y + 1),
            ]
            .into_iter()
            .filter(|(a, b)| *a >= 0 && *b >= 0)
            .map(|(a, b)| (a as usize, b as usize))
        })
        .collect()
}

impl Day3 {
    fn part1(self) -> u32 {
        self.symbols
            .into_iter()
            .filter_map(|loc| self.numbers.get(&loc))
            .flatten()
            .sum()
    }

    fn part2(self) -> u32 {
        self.potential_gears
            .into_iter()
            .filter_map(|gear| {
                if let Some(nums) = self.numbers.get(&gear) {
                    if nums.len() == 2 {
                        return Some(nums.iter().product::<u32>());
                    }
                }
                None
            })
            .sum()
    }
}

impl From<AOCInput> for Day3 {
    fn from(value: AOCInput) -> Self {
        let mut symbols = HashSet::<(usize, usize)>::default();
        let mut numbers = HashMap::<(usize, usize), Vec<u32>>::default();
        let mut potential_gears = HashSet::<(usize, usize)>::default();

        for (i, line) in value.0.lines().enumerate() {
            let mut current: Option<((usize, usize), String)> = None;
            for (j, char) in line.char_indices() {
                if char.is_numeric() {
                    if let Some(current) = current.as_mut() {
                        current.1.push(char);
                    } else {
                        current.replace(((i, j), String::from(char)));
                    }
                } else {
                    if let Some(current) = current.take() {
                        for loc in surrounding_values(current.0, current.1.len()) {
                            numbers
                                .entry(loc)
                                .or_default()
                                .push(current.1.parse().unwrap());
                        }
                    }
                    if char == '*' {
                        potential_gears.insert((i, j));
                    }
                    if char != '.' {
                        symbols.insert((i, j));
                    }
                }
            }
            if let Some(current) = current.take() {
                for loc in surrounding_values(current.0, current.1.len()) {
                    numbers
                        .entry(loc)
                        .or_default()
                        .push(current.1.parse().unwrap());
                }
            }
        }

        Self {
            potential_gears,
            symbols,
            numbers,
        }
    }
}

#[test]
fn tester() {
    let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    println!("{:?}", Day3::from(AOCInput(input.to_string())).part1());
    println!("{:?}", Day3::from(AOCInput(input.to_string())).part2());
}

#[test]
fn part1() {
    use crate::AOCInput;
    let input = AOCInput::from("src/day3/part1.txt");

    println!("{:?}", Day3::from(input).part1());
}

#[test]
fn part2() {
    use crate::AOCInput;
    let input = AOCInput::from("src/day3/part1.txt");

    println!("{:?}", Day3::from(input).part2());
}
