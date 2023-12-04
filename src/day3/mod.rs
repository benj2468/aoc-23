use std::collections::{HashMap, HashSet};

use itertools::Itertools;

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

#[derive(Debug)]
struct Day3_2(u32);

impl From<AOCInput> for Day3_2 {
    fn from(value: AOCInput) -> Self {
        let data = &value.0.lines().collect_vec();

        let regex = regex::Regex::new(r"\d+").unwrap();

        let out: u32 = value
            .0
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                regex.find_iter(line).filter_map(move |matc| {
                    let start = matc.start();
                    let x_range = start.saturating_sub(1)
                        ..std::cmp::min(start + matc.len() + 1, data[0].len());
                    let y_range = i.saturating_sub(1)..std::cmp::min(i + 2, data.len());

                    x_range
                        .cartesian_product(y_range)
                        .into_iter()
                        .any(|(x, y)| {
                            data[y][x..x + 1]
                                .chars()
                                .next()
                                .map(|c| !c.is_numeric() && c != '.')
                                .unwrap_or_default()
                        })
                        .then_some(matc.as_str().parse::<u32>().unwrap())
                })
            })
            .sum();

        Self(out)
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
    // println!("{:?}", Day3::from(AOCInput(input.to_string())).part2());

    println!("{:?}", Day3_2::from(AOCInput(input.to_string())));
}

#[test]
fn part1() {
    use crate::AOCInput;
    let input = AOCInput::from("src/day3/part1.txt");

    println!("{:?}", Day3::from(input.clone()).part1());
    println!("{:?}", Day3_2::from(input));
}

#[test]
fn part2() {
    use crate::AOCInput;
    let input = AOCInput::from("src/day3/part1.txt");

    println!("{:?}", Day3::from(input).part2());
}
