use std::collections::HashMap;

use crate::AOCInput;

#[derive(Debug)]
struct Part1(u32);

impl From<AOCInput> for Part1 {
    fn from(value: AOCInput) -> Self {
        let map = value
            .0
            .lines()
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|a| a.parse::<u32>().unwrap())
            .map(|a| (a, a))
            .collect::<HashMap<_, _>>();

        Self(
            *value
                .0
                .split("\n\n")
                .skip(1)
                .fold(map, |res, mappings| {
                    let mut updates = HashMap::<u32, u32>::default();
                    for mapping in mappings.split('\n').skip(1) {
                        let mut iter = mapping.split_whitespace();
                        let d_start = iter.next().unwrap().parse::<u32>().unwrap();
                        let s_start = iter.next().unwrap().parse::<u32>().unwrap();
                        let len = iter.next().unwrap().parse::<u32>().unwrap();

                        for k in res.keys() {
                            if *k >= s_start && *k < s_start.saturating_add(len) {
                                updates.insert(*k, d_start + (k - s_start));
                            }
                        }
                    }

                    res.into_iter()
                        .map(|(k, v)| (*updates.get(&k).unwrap_or(&k), v))
                        .collect()
                })
                .iter()
                .min_by_key(|(k, _)| *k)
                .unwrap()
                .0,
        )
    }
}

#[derive(Debug)]
struct Part2(u32);

// (Start, (Mapped Start, Range))
type Ranges = HashMap<u32, (u32, u32)>;

fn merge(
    old: (u32, (u32, u32)),
    new: (u32, (u32, u32)),
) -> (Vec<(u32, (u32, u32))>, Vec<(u32, (u32, u32))>) {
    let (original, (old_d, old_l)) = old;
    let (new_s, (new_d, new_l)) = new;

    if new_s.saturating_add(new_l) < old_d {
        return Default::default();
    }

    if new_s > old_d.saturating_add(old_l) {
        return Default::default();
    }

    if new_s < old_d {
        let removed = old_d - new_s;
        return merge(old, (old_d, ((new_d + removed), new_l - removed)));
    }

    if new_s.saturating_add(new_l) > old_d.saturating_add(old_l) {
        let removed = (new_s.saturating_add(new_l)) - (old_d.saturating_add(old_l));
        return merge(old, (new_s, (new_d, new_l - removed)));
    }

    let rem_before = new_s - old_d;
    let rem_after = old_d.saturating_add(old_l) - new_s.saturating_add(new_l);

    let new = [(original + rem_before, (new_d, new_l))]
        .into_iter()
        .filter(|(_, (_, d))| *d > 0)
        .collect();

    let existing = [
        (original, (old_d, rem_before)),
        (
            original + rem_before + new_l,
            (
                old_d.saturating_add(rem_before).saturating_add(new_l),
                rem_after,
            ),
        ),
    ]
    .into_iter()
    .filter(|(_, (_, d))| *d > 0)
    .collect();

    (new, existing)
}

fn merge_ranges(old: Ranges, new: Ranges) -> Ranges {
    let mut merged = Ranges::default();

    for a in old.into_iter() {
        let mut found = false;
        for b in new.clone().into_iter() {
            let (new, existing) = merge(a, b);
            for c in new {
                found = true;
                merged.insert(c.0, c.1);
            }
            for e in existing {
                merged.entry(e.0).or_insert(e.1);
            }
        }
        if !found {
            merged.insert(a.0, a.1);
        }
    }

    merged
}

impl From<AOCInput> for Part2 {
    fn from(value: AOCInput) -> Self {
        let mut map = value.0.lines().next().unwrap().split_whitespace().skip(1);

        let mut res = HashMap::<u32, (u32, u32)>::default();

        while let Some(s) = map.next() {
            let f = s.parse::<u32>().unwrap();
            let l = map.next().unwrap().parse::<u32>().unwrap();
            res.insert(f, (f, l));
        }

        value.0.split("\n\n").skip(1).for_each(|mappings| {
            let mut updates = HashMap::<u32, (u32, u32)>::default();
            for mapping in mappings.split('\n').skip(1) {
                let mut iter = mapping.split_whitespace();
                let d_start = iter.next().unwrap().parse::<u32>().unwrap();
                let s_start = iter.next().unwrap().parse::<u32>().unwrap();
                let len = iter.next().unwrap().parse::<u32>().unwrap();

                updates.insert(s_start, (d_start, len));
            }

            res = merge_ranges(res.clone(), updates);
        });

        Self(res.into_iter().map(|(_, (a, _))| a).min().unwrap())
    }
}

#[test]
fn tester() {
    let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    println!("{:?}", Part1::from(AOCInput(input.to_string())));
    println!("{:?}", Part2::from(AOCInput(input.to_string())));
}

#[test]
fn part1() {
    let input = AOCInput::from("src/day5/input.txt");

    println!("{:?}", Part1::from(input));
}

#[test]
fn part2() {
    let input = AOCInput::from("src/day5/input.txt");

    println!("{:?}", Part2::from(input));
}
