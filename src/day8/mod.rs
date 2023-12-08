use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use crate::AOCInput;

#[derive(Debug)]
struct Part1(u32);

impl From<AOCInput> for Part1 {
    fn from(value: AOCInput) -> Self {
        let number_regex = Regex::new(r"\w+").unwrap();
        let mut lines = value.0.lines();

        let steps = lines
            .next()
            .unwrap()
            .chars()
            .map(|a| match a {
                'R' => 1,
                _ => 0,
            })
            .collect_vec();

        let graph = lines
            .skip(1)
            .map(|line| {
                let mut parts = number_regex.find_iter(line);

                let node = parts.next().unwrap().as_str();

                let n1 = parts.next().unwrap().as_str();
                let n2 = parts.next().unwrap().as_str();

                (node, vec![n1, n2])
            })
            .collect::<HashMap<&str, Vec<&str>>>();

        // println!("{:?}", (steps, graph));

        let graph = &graph;

        let best_path = pathfinding::directed::dijkstra::dijkstra(
            &("AAA", 0_usize),
            |&(node, mut step)| {
                let next = graph.get(node).unwrap().get(steps[step]).unwrap();

                step += 1;
                step %= steps.len();

                vec![((*next, step), 1)]
            },
            |&(node, _)| node == "ZZZ",
        )
        .unwrap();

        Self(best_path.1 as u32)
    }
}

#[derive(Debug)]
struct Part2(usize);

impl From<AOCInput> for Part2 {
    fn from(value: AOCInput) -> Self {
        let number_regex = Regex::new(r"\w+").unwrap();
        let mut lines = value.0.lines();

        let steps = lines
            .next()
            .unwrap()
            .chars()
            .map(|a| match a {
                'R' => 1,
                _ => 0,
            })
            .collect_vec();

        let graph = lines
            .skip(1)
            .map(|line| {
                let mut parts = number_regex.find_iter(line);

                let node = parts.next().unwrap().as_str();

                let n1 = parts.next().unwrap().as_str();
                let n2 = parts.next().unwrap().as_str();

                (node, vec![n1, n2])
            })
            .collect::<HashMap<&str, Vec<&str>>>();

        let res = graph
            .keys()
            .filter(|a| a.ends_with('A'))
            .map(|start| {
                let mut curr = *start;
                let mut step = 0;

                let mut visited = HashMap::<(usize, &str), usize>::default();

                while !visited.contains_key(&(step % steps.len(), curr)) {
                    if curr.ends_with('Z') {
                        visited.insert((step % steps.len(), curr), step);
                    }

                    curr = graph
                        .get(curr)
                        .unwrap()
                        .get(steps[step % steps.len()])
                        .unwrap();
                    step += 1;
                }
                visited
            })
            .flat_map(|a| a.into_values().collect_vec().into_iter().rev())
            .into_iter()
            .fold(1, num::integer::lcm);

        Self(res)
    }
}

#[test]
fn tester() {
    let input = r#"LR

    AAA = (AAB, XXX)
    AAB = (XXX, AAZ)
    AAZ = (AAB, XXX)
    BBA = (BBB, XXX)
    BBB = (BBC, BBC)
    BBC = (BBZ, BBZ)
    BBZ = (BBB, BBB)
    XXX = (XXX, XXX)"#;

    // println!("{:?}", Part1::from(AOCInput(input.to_string())));
    println!("{:?}", Part2::from(AOCInput(input.to_string())));
}

#[test]
fn parts() {
    let input = AOCInput::from("src/day8/input.txt");

    println!("{:?}", Part1::from(input.clone()));
    println!("{:?}", Part2::from(input));
}
