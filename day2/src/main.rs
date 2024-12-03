const TEST_INPUT: &str = include_str!("test.txt");
const FULL_INPUT: &str = include_str!("full.txt");

fn is_report_safe(report: Vec<isize>) -> bool {
    if !report.is_sorted() && !report.iter().rev().is_sorted() {
        return false;
    }

    let mut report = report.into_iter();
    let mut last = report.next().unwrap();
    for item in report {
        let diff = (last - item).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
        last = item;
    }

    true
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let report: Vec<_> = line
                .split_whitespace()
                .map(|it| it.parse::<isize>().unwrap())
                .collect();

            is_report_safe(report)
        })
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let report: Vec<_> = line
                .split_whitespace()
                .map(|it| it.parse::<isize>().unwrap())
                .collect();

            if is_report_safe(report.clone()) {
                return true;
            }

            (0..report.len()).any(|i| {
                let mut r_prime = report.clone();
                r_prime.remove(i);
                is_report_safe(r_prime)
            })
        })
        .count()
}

fn main() {
    println!("P1 - test: {} full: {}", part1(TEST_INPUT), part1(FULL_INPUT));
    println!("P2 - test: {} full: {}", part2(TEST_INPUT), part2(FULL_INPUT));
}

