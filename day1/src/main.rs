use std::collections::HashMap;

const TEST_INPUT: &str = include_str!("test.txt");
const FULL_INPUT: &str = include_str!("full.txt");

fn part1(input: &str) -> isize {
    let (mut a, mut b) = input
        .lines()
        .map(|line| {
            let mut segments = line.split_whitespace();
            let a: isize = segments.next().unwrap().parse().unwrap();
            let b: isize = segments.next().unwrap().parse().unwrap();

            (a, b)
        })
        .collect::<(Vec<_>, Vec<_>)>();

    a.sort_unstable();
    b.sort_unstable();

    a.iter().zip(b.iter()).map(|(a, b)| (a - b).abs()).sum()
}

fn part2(input: &str) -> isize {
    let (a, b) = input
        .lines()
        .map(|line| {
            let mut segments = line.split_whitespace();
            let a: isize = segments.next().unwrap().parse().unwrap();
            let b: isize = segments.next().unwrap().parse().unwrap();

            (a, b)
        })
        .collect::<(Vec<_>, Vec<_>)>();

    let freq = b
        .into_iter()
        .fold(HashMap::<isize, isize>::new(), |mut m, x| {
            *m.entry(x).or_default() += 1;
            m
        });

    a.iter()
        .fold(0, |acc, item| acc + (item * freq.get(item).unwrap_or(&0)))
}

fn main() {
    println!("P1 - test: {} full: {}", part1(TEST_INPUT), part1(FULL_INPUT));
    println!("P2 - test: {} full: {}", part2(TEST_INPUT), part2(FULL_INPUT));
}
