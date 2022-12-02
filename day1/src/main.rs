fn part_one(input: &str) -> u32 {
    let mut highest_calories = 0;

    let mut current_calories = 0;
    for line in input.lines() {
        if !line.is_empty() {
            let calories: u32 = line.parse().expect("received illegal input");
            current_calories += calories;
            continue;
        }

        if current_calories > highest_calories {
            highest_calories = current_calories;
        }

        current_calories = 0;
    }

    highest_calories
}

fn part_two(input: &str) -> u32 {
    let mut elves: Vec<u32> = vec![0];

    let mut elf_index = 0;
    for line in input.lines() {
        if !line.is_empty() {
            let calories: u32 = line.parse().expect("received illegal input");
            elves[elf_index] += calories;
            continue;
        }

        elves.push(0);
        elf_index += 1;
    }

    elves.sort_unstable();
    elves[elves.len() - 3..].iter().sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("highest calories: {}", part_one(input));

    println!("highest combined calories: {}", part_two(input));
}
