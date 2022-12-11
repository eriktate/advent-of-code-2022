#[derive(Debug)]
struct Move {
    count: u32,
    src: usize,
    dst: usize,
}

impl Move {
    fn parse(input: &str) -> Move {
        let parts: Vec<&str> = input.split(" ").collect();
        Move {
            count: parts[1].parse().expect("expected move count"),
            src: parts[3].parse().expect("expected src stack"),
            dst: parts[5].parse().expect("expected dst stack"),
        }
    }

    fn apply(&self, stacks: &mut Vec<Vec<char>>) {
        for _ in 0..self.count {
            match stacks[self.src - 1].pop() {
                Some(moved) => stacks[self.dst - 1].push(moved),
                None => (),
            }
        }
    }

    fn apply_two(&self, stacks: &mut Vec<Vec<char>>) {
        let mut batch: Vec<char> = Vec::new();

        for _ in 0..self.count {
            match stacks[self.src - 1].pop() {
                Some(moved) => batch.push(moved),
                None => (),
            }
        }
        batch.reverse();
        for ch in batch.iter() {
            stacks[self.dst - 1].push(*ch);
        }
    }
}

fn run(input: &str, part_two: bool) {
    // let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let lines = input.lines();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut parsing_stacks = true;
    let mut skip_blank = true;

    for line in lines {
        if parsing_stacks {
            for (idx, ch) in line.chars().skip(1).step_by(4).enumerate() {
                // found labels, reverse stacks and move on to parsing moves
                if ch.is_numeric() {
                    for stack in &mut stacks {
                        stack.reverse();
                    }

                    parsing_stacks = false;
                    break;
                }

                if stacks.len() < idx + 1 {
                    stacks.push(Vec::new());
                }

                if ch.is_alphabetic() {
                    stacks[idx].push(ch);
                }
            }
            continue;
        }

        if skip_blank {
            skip_blank = false;
            continue;
        }

        let mov = Move::parse(line);
        if part_two {
            mov.apply_two(&mut stacks);
        } else {
            mov.apply(&mut stacks);
        }
    }

    let result: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    println!("result: {}", result);
}

fn main() {
    let input = include_str!("../input.txt");

    run(input, false);
    run(input, true);
}
