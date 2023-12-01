use std::io;

static DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let mut total = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let mut digits = line.chars().enumerate().filter_map(|(idx, c)| {
            c.to_digit(10).or_else(|| {
                DIGITS
                    .iter()
                    .enumerate()
                    .find(|(_, &s)| line[idx..].starts_with(s))
                    .map(|(idx, _)| idx as u32)
            })
        });

        let first = digits.next();
        total += first.unwrap_or(0) * 10;
        total += digits.last().or(first).unwrap_or(0);
    }

    println!("Answer: {total}");
}
