use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn part1() -> isize {
    let (mut a, mut b) = INPUT
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

fn part2() -> isize {
    let (a, b) = INPUT
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
    println!("{}", part1());
    println!("{}", part2());
}
