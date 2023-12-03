use std::io;

fn is_symbol(c: u8) -> bool {
    !c.is_ascii_digit() && c != b'.'
}

fn check_adjacent(x: usize, y: usize, board: &[Vec<u8>]) -> bool {
    let xmax = board[y].len() - 1;
    let ymax = board.len() - 1;

    // if x is zero lowest is zero, otherwise x - 1
    let min_x = x.saturating_sub(1);
    let max_x = (x + 1).min(xmax);
    let min_y = y.saturating_sub(1);
    let max_y = (y + 1).min(ymax);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if is_symbol(board[y][x]) {
                return true;
            }
        }
    }

    false
}

fn score_board(board: Vec<Vec<u8>>) -> i32 {
    let mut sum = 0;
    let mut num_stack = Vec::new();
    for y in 0..board.len() {
        let mut can_add = false;
        for x in 0..board[y].len() {
            let c = board[y][x];

            if !c.is_ascii_digit() {
                if can_add {
                    let mut base = 1;
                    for num in num_stack.iter().rev() {
                        sum += num * base;
                        base *= 10;
                    }
                }

                can_add = false;
                num_stack.clear();
                continue;
            }

            let c = c - b'0';
            num_stack.push(c as i32);

            // check adjacents
            can_add |= check_adjacent(x, y, &board);
        }
        if can_add {
            let mut base = 1;
            for num in num_stack.iter().rev() {
                sum += num * base;
                base *= 10;
            }
        }
        num_stack.clear();
    }

    sum
}

fn main() {
    let board = io::stdin()
        .lines()
        .map(|line| line.map(|s| s.into_bytes()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let sum = score_board(board);
    println!("Answer {sum}");
}

#[cfg(test)]
mod test {
    use std::fs;

    #[test]
    fn test_main_input() {
        let input: Vec<Vec<u8>> = fs::read_to_string("inputs/day3.txt")
            .unwrap()
            .lines()
            .map(|line| line.to_owned().into_bytes())
            .collect();

        let output = super::score_board(input);
        println!("Output: {output}");
    }

    #[test]
    fn test_test_input() {
        let input: Vec<Vec<u8>> = fs::read_to_string("inputs/day3_test.txt")
            .unwrap()
            .lines()
            .map(|line| line.to_owned().into_bytes())
            .collect();

        let output = super::score_board(input);
        assert_eq!(output, 4361);
    }
}
