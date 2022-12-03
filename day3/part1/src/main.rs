use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let values = ('a'..='z')
        .chain('A'..='Z')
        .zip(1..=52)
        .collect::<HashMap<char, usize>>();

    println!(
        "{}",
        fs::read_to_string("input.txt")
            .unwrap()
            .split_whitespace()
            .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
            .map(|(comp1, comp2)| {
                comp1
                    .chars()
                    .collect::<HashSet<char>>()
                    .intersection(&comp2.chars().collect::<HashSet<char>>())
                    .next()
                    .unwrap()
                    .to_owned()
            })
            .map(|letter| values.get(&letter).unwrap())
            .sum::<usize>()
    )
}
