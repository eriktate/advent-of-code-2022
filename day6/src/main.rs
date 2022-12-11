use std::collections::HashSet;
use std::iter::FromIterator;

// const SEQUENCE_LEN: usize = 4; // part one
const SEQUENCE_LEN: usize = 14; // part two

fn append(window: &mut [char; SEQUENCE_LEN], ch: char) {
    for idx in 1..SEQUENCE_LEN {
        window[idx - 1] = window[idx];
    }

    window[SEQUENCE_LEN - 1] = ch;
}

fn part_one(input: &str) -> usize {
    let mut window = ['\0'; SEQUENCE_LEN];

    for (idx, ch) in input.chars().enumerate() {
        append(&mut window, ch);
        let set: HashSet<char> = HashSet::from_iter(window.iter().cloned());
        if set.len() == SEQUENCE_LEN && idx >= SEQUENCE_LEN {
            return idx + 1;
        }
    }

    0
}

fn main() {
    let input = include_str!("../input.txt");

    println!("part one: {}", part_one(input));
}
