use crate::AOCInput;

fn solve_history_one(history: Vec<isize>) -> isize {
    if history.iter().all(|a| *a == 0) {
        0
    } else {
        history.last().unwrap()
            + solve_history_one(history.as_slice().windows(2).map(|w| w[1] - w[0]).collect())
    }
}

#[derive(Debug)]
struct Part1(isize);

impl From<AOCInput> for Part1 {
    fn from(value: AOCInput) -> Self {
        Self(
            value
                .0
                .lines()
                .map(|a| {
                    solve_history_one(
                        a.split_whitespace()
                            .map(|a| a.parse::<isize>().unwrap())
                            .collect(),
                    )
                })
                .sum(),
        )
    }
}

fn solve_history_two(history: Vec<isize>) -> isize {
    if history.iter().all(|a| *a == 0) {
        0
    } else {
        history.first().unwrap()
            - solve_history_two(history.as_slice().windows(2).map(|w| w[1] - w[0]).collect())
    }
}

#[derive(Debug)]
struct Part2(isize);

impl From<AOCInput> for Part2 {
    fn from(value: AOCInput) -> Self {
        Self(
            value
                .0
                .lines()
                .map(|a| {
                    solve_history_two(
                        a.split_whitespace()
                            .map(|a| a.parse::<isize>().unwrap())
                            .collect(),
                    )
                })
                .sum(),
        )
    }
}

#[test]
fn tester() {
    let input = r#"0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45"#;

    println!("{:?}", Part1::from(AOCInput(input.to_string())));
    println!("{:?}", Part2::from(AOCInput(input.to_string())));
}

#[test]
fn parts() {
    let input = AOCInput::from("src/day9/input.txt");

    println!("{:?}", Part1::from(input.clone()));
    println!("{:?}", Part2::from(input));
}
