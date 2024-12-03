use regex::{Captures, Regex};

const TEST_INPUT: &str = include_str!("test.txt");
const TEST_INPUT_2: &str = include_str!("test2.txt");
const FULL_INPUT: &str = include_str!("full.txt");

fn mul(cap: Captures) -> isize {
    cap[1].parse::<isize>().expect("Invalid integer")
        * cap[2].parse::<isize>().expect("Invalid integer")
}

fn part1(input: &str) -> isize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Invalid regex");

    re.captures_iter(input)
        .map(mul)
        .sum()
}

fn part2(input: &str) -> isize {
    let re = Regex::new(r"(?:mul\((\d{1,3}),(\d{1,3})\))|do\(\)|don't\(\)").expect("Invalid regex");

    // This probably isn't the most efficient or elegant solution,
    // but for this size of input, we don't *really* need a more sophisticated parser or state machine.
    re.captures_iter(input)
        .scan(true, |mul_enabled, cap| {
            if &cap[0] == "do()" {
                *mul_enabled = true;
            } else if &cap[0] == "don't()" {
                *mul_enabled = false;
            } else if *mul_enabled {
                return Some(mul(cap))
            }

            // No op; but summing with zero is identity ;)
            Some(0)
        })
        .sum()
}

fn main() {
    println!(
        "P1 - test: {} full: {}",
        part1(TEST_INPUT),
        part1(FULL_INPUT)
    );
    println!(
        "P2 - test: {} full: {}",
        part2(TEST_INPUT_2),
        part2(FULL_INPUT)
    );
}
