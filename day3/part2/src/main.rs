use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let values = ('a'..='z')
        .chain('A'..='Z')
        .zip(1..=52)
        .collect::<HashMap<char, usize>>();

    let binding = fs::read_to_string("input.txt").unwrap();
    let mut rucksacks = binding
        .split_whitespace()
        .map(|rucksack| rucksack.chars().collect::<HashSet<char>>());

    let mut sum = 0;
    while let (Some(r1), Some(r2), Some(r3)) =
        (rucksacks.next(), rucksacks.next(), rucksacks.next())
    {
        sum += values
            .get(
                &r1.intersection(&r2)
                    .map(|c| c.to_owned())
                    .collect::<HashSet<char>>()
                    .intersection(&r3)
                    .next()
                    .unwrap(),
            )
            .unwrap();
    }
    println!("{}", sum);
}
