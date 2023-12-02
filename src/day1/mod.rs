#[derive(Debug)]
pub struct Day1(u32);

mod part1 {
    use itertools::{FoldWhile, Itertools};

    use crate::AOCInput;

    use super::Day1;

    impl From<AOCInput> for Day1 {
        fn from(value: AOCInput) -> Self {
            Day1(
                value
                    .as_ref()
                    .lines()
                    .map(|line| -> u32 {
                        let first = line
                            .chars()
                            .fold_while(String::default(), |mut s, c| {
                                if c.is_numeric() {
                                    return FoldWhile::Done(c.to_string());
                                } else {
                                    s.push(c);
                                    if let Some(i) = s.find("one") {
                                        return FoldWhile::Done(1.to_string());
                                    } else if let Some(i) = s.find("two") {
                                        return FoldWhile::Done(2.to_string());
                                    } else if let Some(i) = s.find("three") {
                                        return FoldWhile::Done(3.to_string());
                                    } else if let Some(i) = s.find("four") {
                                        return FoldWhile::Done(4.to_string());
                                    } else if let Some(i) = s.find("five") {
                                        return FoldWhile::Done(5.to_string());
                                    } else if let Some(i) = s.find("six") {
                                        return FoldWhile::Done(6.to_string());
                                    } else if let Some(i) = s.find("seven") {
                                        return FoldWhile::Done(7.to_string());
                                    } else if let Some(i) = s.find("eight") {
                                        return FoldWhile::Done(8.to_string());
                                    } else if let Some(i) = s.find("nine") {
                                        return FoldWhile::Done(9.to_string());
                                    }
                                }

                                FoldWhile::Continue(s)
                            })
                            .into_inner();

                        let last = line
                            .chars()
                            .rev()
                            .fold_while(String::default(), |mut s, c| {
                                if c.is_numeric() {
                                    return FoldWhile::Done(c.to_string());
                                } else {
                                    s.insert(0, c);
                                    if let Some(i) = s.find("one") {
                                        return FoldWhile::Done(1.to_string());
                                    } else if let Some(i) = s.find("two") {
                                        return FoldWhile::Done(2.to_string());
                                    } else if let Some(i) = s.find("three") {
                                        return FoldWhile::Done(3.to_string());
                                    } else if let Some(i) = s.find("four") {
                                        return FoldWhile::Done(4.to_string());
                                    } else if let Some(i) = s.find("five") {
                                        return FoldWhile::Done(5.to_string());
                                    } else if let Some(i) = s.find("six") {
                                        return FoldWhile::Done(6.to_string());
                                    } else if let Some(i) = s.find("seven") {
                                        return FoldWhile::Done(7.to_string());
                                    } else if let Some(i) = s.find("eight") {
                                        return FoldWhile::Done(8.to_string());
                                    } else if let Some(i) = s.find("nine") {
                                        return FoldWhile::Done(9.to_string());
                                    }
                                }

                                FoldWhile::Continue(s)
                            })
                            .into_inner();

                        (format!("{first}{last}")).parse().unwrap()
                    })
                    .sum(),
            )
        }
    }
}

#[test]
fn test() {
    use crate::AOCInput;
    let input = AOCInput(
        r#"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"#
            .to_string(),
    );

    println!("{:?}", Day1::from(input));
}

#[test]
fn part1() {
    use crate::AOCInput;
    let input = AOCInput::from("src/day1/part1.txt");

    println!("{:?}", Day1::from(input));
}
