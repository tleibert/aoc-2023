use std::io;

use aoc_2023::FirstAndLast;

static DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let mut total = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let (first, last) = line
            .chars()
            .enumerate()
            .filter_map(|(idx, c)| {
                c.to_digit(10).or_else(|| {
                    DIGITS
                        .iter()
                        .enumerate()
                        .find(|(_, &s)| line[idx..].starts_with(s))
                        .map(|(idx, _)| idx as u32)
                })
            })
            .first_and_last()
            .unwrap_or((0, 0));

        total += first * 10;
        total += last;
    }

    println!("Answer: {total}");
}
