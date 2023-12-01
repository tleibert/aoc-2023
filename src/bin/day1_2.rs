use std::{
    collections::HashMap,
    io::{self},
};

fn main() {
    let digit_map = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut total = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let digits: Vec<_> = line
            .chars()
            .enumerate()
            .filter_map(|(idx, c)| {
                c.to_digit(10).or_else(|| {
                    digit_map
                        .iter()
                        .find(|(k, _)| line[idx..].starts_with(*k))
                        .map(|(_, v)| *v)
                })
            })
            .collect();

        total += digits.first().unwrap_or(&0) * 10;
        total += digits.last().unwrap_or(&0);
    }

    println!("Answer: {total}");
}
