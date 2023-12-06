use itertools::Itertools;

use crate::AOCInput;

#[derive(Debug)]
struct Part1(u32);

fn ways_to_win(time: usize, record: usize) -> usize {
    let mut decreasing = None;
    let mut res = 0;
    for t in 0..time + 1 {
        let score = (time - t) * t;
        if score > record {
            res += 1;
        }

        if let Some(last) = decreasing.as_mut() {
            if *last > score {
                break;
            }
            *last = score;
        }
    }
    res
}

impl From<AOCInput> for Part1 {
    fn from(value: AOCInput) -> Self {
        let mut lines = value.0.lines();

        let times = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|a| a.parse::<usize>().unwrap());
        let distances = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|a| a.parse::<usize>().unwrap());

        Self(
            times
                .zip(distances)
                .map(|(t, d)| ways_to_win(t, d) as u32)
                .product(),
        )
    }
}

#[derive(Debug)]
struct Part2(usize);

impl From<AOCInput> for Part2 {
    fn from(value: AOCInput) -> Self {
        let mut lines = value.0.lines();

        let t = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .join("")
            .parse::<usize>()
            .unwrap();
        let record = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .join("")
            .parse::<usize>()
            .unwrap();

        Self(ways_to_win(t, record) as usize)
    }
}

#[test]
fn tester() {
    let input = r#"Time:      7  15   30
    Distance:  9  40  200"#;

    println!("{:?}", Part1::from(AOCInput(input.to_string())));
    println!("{:?}", Part2::from(AOCInput(input.to_string())));
}

#[test]
fn part1() {
    let input = AOCInput::from("src/day6/input.txt");

    println!("{:?}", Part1::from(input));
}

#[test]
fn part2() {
    let input = AOCInput::from("src/day6/input.txt");

    println!("{:?}", Part2::from(input));
}
