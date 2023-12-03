use std::io;

#[derive(Eq, PartialEq, Clone)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    fn from_str(s: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for entry in s.trim().split(',') {
            let entry = entry.trim();
            let (number, color) = entry.split_once(' ').unwrap();
            let number: u32 = number.parse().unwrap();
            let color = match color {
                "red" => &mut red,
                "green" => &mut green,
                "blue" => &mut blue,
                _ => panic!("Unknown color {color}"),
            };
            *color += number;
        }

        Self::new(red, green, blue)
    }

    fn power(self) -> u32 {
        self.red * self.green * self.blue
    }

    fn merge(self, other: Self) -> Self {
        Self::new(
            self.red.max(other.red),
            self.green.max(other.green),
            self.blue.max(other.blue),
        )
    }
}

fn main() {
    let total: u32 = io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (_, line) = line.split_once(':').unwrap();
            let needed = line.trim().split(';').fold(Cubes::new(0, 0, 0), |acc, s| {
                let new = Cubes::from_str(s);
                acc.merge(new)
            });

            needed.power()
        })
        .sum();

    println!("Answer {total}")
}
