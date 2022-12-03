use std::collections::HashSet;
use std::fs;

struct Rucksack {
    compartment1: String,
    compartment2: String,
}

impl Rucksack {
    fn new(line: &str) -> Rucksack {
        let (s1, s2) = line.split_at(line.len() / 2);
        Rucksack {
            compartment1: String::from(s1),
            compartment2: String::from(s2),
        }
    }
}

struct ElfGroup {
    rucksack1: String,
    rucksack2: String,
    rucksack3: String,
}

impl ElfGroup {
    fn new(rucksacks: &[&str]) -> ElfGroup {
        ElfGroup {
            rucksack1: String::from(rucksacks[0]),
            rucksack2: String::from(rucksacks[1]),
            rucksack3: String::from(rucksacks[2]),
        }
    }
}

fn find_packing_mistake(rucksack: &Rucksack) -> char {
    let first_compartment: HashSet<char> = rucksack.compartment1.chars().collect();
    let second_compartment: HashSet<char> = rucksack.compartment2.chars().collect();
    *first_compartment
        .intersection(&second_compartment)
        .next()
        .unwrap()
}

fn find_badge(group: &ElfGroup) -> char {
    let first_rucksack: HashSet<char> = group.rucksack1.chars().collect();
    let second_rucksack: HashSet<char> = group.rucksack2.chars().collect();
    let third_rucksack: HashSet<char> = group.rucksack3.chars().collect();
    let first_and_second: HashSet<char> = first_rucksack
        .intersection(&second_rucksack)
        .map(|item| *item)
        .collect();
    *first_and_second
        .intersection(&third_rucksack)
        .next()
        .unwrap()
}

fn as_priority(c: &char) -> u32 {
    let n = *c as u32;
    if c.is_uppercase() {
        n - 'A' as u32 + 27
    } else {
        n - 'a' as u32 + 1
    }
}

pub fn solve() {
    let input = fs::read_to_string("resources/day3.txt").unwrap();

    let rucksacks: Vec<Rucksack> = input.lines().map(Rucksack::new).collect();

    let packing_mistakes: Vec<char> = rucksacks.iter().map(find_packing_mistake).collect();

    let mistake_priority: u32 = packing_mistakes.iter().map(as_priority).sum();

    println!("{}", mistake_priority);

    let input_vec: Vec<&str> = input.lines().collect();

    let elf_groups: Vec<ElfGroup> = input_vec.chunks(3).map(ElfGroup::new).collect();

    let badges: Vec<char> = elf_groups.iter().map(find_badge).collect();

    let badge_priority: u32 = badges.iter().map(as_priority).sum();

    println!("{}", badge_priority);
}
