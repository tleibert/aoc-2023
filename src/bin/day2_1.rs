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

    fn contains(&self, other: &Self) -> bool {
        other.red <= self.red && other.green <= self.green && other.blue <= self.blue
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
}

fn main() {
    let target = Cubes::new(12, 13, 14);

    let total: usize = io::stdin()
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            let line = line.unwrap();
            let idx = idx + 1;

            let (_, line) = line.split_once(':').unwrap();
            let possible = line
                .trim()
                .split(';')
                .all(|s| target.contains(&Cubes::from_str(s)));

            possible.then_some(idx)
        })
        .sum();

    println!("Answer {total}")
}
