use std::io;

fn main() {
    let mut total = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let digits: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();

        total += digits.first().unwrap_or(&0) * 10;
        total += digits.last().unwrap_or(&0);
    }

    println!("Answer: {total}");
}
