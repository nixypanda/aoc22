use std::collections::HashSet;

use itertools::Itertools;

const GROUP_SIZE: usize = 3;

type Rucksack = Vec<char>;
type Compartment = HashSet<char>;
type Item = char;
type ItemPriority = usize;
type Group = Vec<HashSet<char>>;
type Badge = char;

fn parse_input(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|rucksack| rucksack.chars().collect())
        .collect()
}

fn item_priority(item: Item) -> ItemPriority {
    if item.is_lowercase() {
        item as usize - 'a' as usize + 1
    } else if item.is_uppercase() {
        item as usize - 'A' as usize + 27
    } else {
        panic!("Unrecognised item!")
    }
}

fn priority_of_overlapping_item(rucksack: &Rucksack) -> ItemPriority {
    // let rucksack: Rucksack = rucksack.chars().collect();
    let compartment_size = rucksack.len() / 2;

    let first_compartment: Compartment = rucksack[..compartment_size].iter().copied().collect();
    let second_compartment: Compartment = rucksack[compartment_size..].iter().copied().collect();

    first_compartment
        .intersection(&second_compartment)
        .copied()
        .into_iter()
        .map(item_priority)
        .sum::<usize>()
}

fn part1(rucksacks: &[Rucksack]) -> usize {
    rucksacks.iter().map(priority_of_overlapping_item).sum()
}

fn identify_group_badges(rucksacks: Group) -> Vec<Badge> {
    rucksacks
        .into_iter()
        .reduce(|group, rucksack| -> HashSet<_> {
            group.intersection(&rucksack).into_iter().copied().collect()
        })
        .unwrap()
        .into_iter()
        .collect::<Vec<Badge>>()
}

fn part2(rucksacks: &[Rucksack]) -> usize {
    rucksacks
        .iter()
        .map(|rucksack| rucksack.iter().copied().collect::<HashSet<char>>())
        .chunks(GROUP_SIZE)
        .into_iter()
        .map(|rucksacks| {
            identify_group_badges(rucksacks.collect())
                .iter()
                .map(|item| item_priority(*item))
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let rucksacks = parse_input(include_str!("../../data/day03.txt"));
    println!("Day 03 - Part 01: {}", part1(&rucksacks));
    println!("Day 03 - Part 01: {}", part2(&rucksacks));
}
