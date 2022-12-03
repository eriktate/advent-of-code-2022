use std::collections::HashSet;

fn find_shared_item(rucksack: &str) -> Option<char> {
    let mut found_items: HashSet<char> = HashSet::new();
    let items: Vec<char> = rucksack.chars().collect();
    let first_compartment = &items[0..items.len() / 2];
    let second_compartment = &items[items.len() / 2..];

    for item in first_compartment.iter() {
        found_items.insert(*item);
    }

    for item in second_compartment.iter() {
        if found_items.contains(item) {
            return Some(*item);
        }
    }

    None
}

fn find_badge(group: Vec<&str>) -> Option<char> {
    let mut first_rucksack: HashSet<char> = HashSet::new();
    let mut common_items: HashSet<char> = HashSet::new();

    if group.len() != 3 {
        return None;
    }

    for item in group[0].chars() {
        first_rucksack.insert(item);
    }

    for item in group[1].chars() {
        if first_rucksack.contains(&item) {
            common_items.insert(item);
        }
    }

    for item in group[2].chars() {
        if common_items.contains(&item) {
            return Some(item);
        }
    }

    None
}

fn get_priority(item: char) -> Option<i32> {
    let val = item as i32;
    if val > 96 {
        return Some(val - 96);
    }

    if val > 64 {
        return Some(val - 64 + 26);
    }

    None
}

fn part_one(input: &str) -> i32 {
    let mut total_priority = 0;
    for rucksack in input.lines() {
        let shared_item = find_shared_item(rucksack).expect("invalid input: no shared items");
        total_priority += get_priority(shared_item).expect("invalid input: non-alpha items");
    }

    total_priority
}

fn part_two(input: &str) -> i32 {
    let mut total_priority = 0;
    let mut rucksacks = input.lines();

    loop {
        let group: Vec<&str> = rucksacks.by_ref().take(3).collect();
        if group.len() < 3 {
            break;
        }

        let badge = find_badge(group).expect("invalid input: no badge found");
        total_priority += get_priority(badge).expect("invalid input: non-alpha items");
    }

    total_priority
}

fn main() {
    let input = include_str!("../input.txt");

    println!("part one: {}", part_one(&input));
    println!("part two: {}", part_two(&input));
}
