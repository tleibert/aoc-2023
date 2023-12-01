use std::io;

use aoc_2023::FirstAndLast;

fn main() {
    let mut total = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let (first, last) = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .first_and_last()
            .unwrap_or((0, 0));

        total += first * 10;
        total += last;
    }

    println!("Answer: {total}");

    let items: [i32; 1] = [1];
    println!("First and last: {:?}", items.iter().first_and_last());
}
