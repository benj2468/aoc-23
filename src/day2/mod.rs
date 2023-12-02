use crate::day2::part1::{Day2, Game, GamePart};

mod part1 {
    use std::cmp;

    use crate::AOCInput;

    #[derive(PartialEq, Debug, Default)]
    pub struct GamePart {
        pub red: u32,
        pub green: u32,
        pub blue: u32,
    }

    impl PartialOrd<GamePart> for GamePart {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            if self.red <= other.red && self.green <= other.green && self.blue <= other.blue {
                Some(std::cmp::Ordering::Less)
            } else if self.red > other.red || self.green > other.green || self.blue > other.blue {
                Some(std::cmp::Ordering::Greater)
            } else {
                None
            }
        }
    }

    impl From<&str> for GamePart {
        fn from(value: &str) -> Self {
            let cubes = value.split(",");

            let mut res = Self::default();

            for cube in cubes {
                let (n, name) = cube.trim().split_once(" ").unwrap();
                println!("{:?} {:?}", n, name);
                let n = n.parse().unwrap();
                if name == "blue" {
                    res.blue = n;
                } else if name == "green" {
                    res.green = n;
                } else if name == "red" {
                    res.red = n;
                }
            }

            res
        }
    }

    pub struct Game {
        id: u32,
        parts: Vec<GamePart>,
    }

    impl Game {
        fn power(&self) -> u32 {
            let part = self.parts.iter().fold(GamePart::default(), |mut res, cur| {
                res.blue = std::cmp::max(cur.blue, res.blue);
                res.red = std::cmp::max(cur.red, res.red);
                res.green = std::cmp::max(cur.green, res.green);
                res
            });

            part.blue * part.green * part.red
        }
    }

    impl From<&str> for Game {
        fn from(value: &str) -> Self {
            let (id, parts) = value.split_once(":").unwrap();
            let parts = parts.split(";").map(|part| GamePart::from(part)).collect();

            Self {
                id: id.trim().split_once(" ").unwrap().1.parse().unwrap(),
                parts,
            }
        }
    }

    #[derive(Debug)]
    pub struct Day2(u32);

    impl From<AOCInput> for Day2 {
        fn from(value: AOCInput) -> Self {
            let max = GamePart {
                blue: 14,
                green: 13,
                red: 12,
            };
            Day2(
                value
                    .as_ref()
                    .lines()
                    .map(Game::from)
                    .map(|game| game.power())
                    // .filter(|game| game.parts.iter().all(|p| p < &max))
                    // .map(|game| game.id)
                    .sum(),
            )
        }
    }
}

#[test]
fn test() {
    use crate::AOCInput;
    let input = AOCInput(
        r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#
            .to_string(),
    );

    println!("{:?}", Day2::from(input));
}

#[test]
fn part1() {
    use crate::AOCInput;
    let input = AOCInput::from("src/day2/part1.txt");

    println!("{:?}", Day2::from(input));
}

#[test]
fn test_1() {
    use crate::AOCInput;
    let part1 = GamePart {
        blue: 10,
        green: 5,
        red: 1,
    };

    let part2 = GamePart {
        blue: 11,
        green: 5,
        red: 1,
    };

    println!("{:?}", part1 < part2);
}
