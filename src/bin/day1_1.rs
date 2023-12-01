use std::io;

fn main() {
    let mut total = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let mut digits = line.chars().filter_map(|c| c.to_digit(10));

        let first = digits.next();
        total += first.unwrap_or(0) * 10;
        total += digits.last().or(first).unwrap_or(0);
    }

    println!("Answer: {total}");
}
