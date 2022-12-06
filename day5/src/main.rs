use std::fs;

fn main() {
    let mut crates: Vec<Vec<char>> = Vec::new();
    crates.push(Vec::new());

    let binding = fs::read_to_string("input.txt").unwrap();
    let lines = binding.lines().collect::<Vec<&str>>();

    let cols = lines[0].len() / 4 + 1;
    let rows = lines
        .iter()
        .enumerate()
        .find(|(_index, &s)| s.chars().any(|c| c.is_numeric()))
        .unwrap()
        .0;

    for col in 0..cols {
        let mut c: Vec<char> = Vec::new();
        for row in (0..rows).rev() {
            let char = lines[row].chars().nth(col * 4 + 1).unwrap();
            if char.is_alphabetic() {
                c.push(char);
            }
        }
        crates.push(c);
    }

    let mut input = lines
        .into_iter()
        .skip(rows + 2)
        .flat_map(|line| line.split_whitespace())
        .filter_map(|val| val.parse::<usize>().ok());

    let mut part2crates = crates.clone();
    while let (Some(count), Some(source), Some(dest)) = (input.next(), input.next(), input.next()) {
        let mut tempstack: Vec<char> = Vec::new();
        for _ in 0..count {
            let val1 = crates[source].pop().unwrap();
            crates[dest].push(val1);
            let val2 = part2crates[source].pop().unwrap();
            tempstack.push(val2);
        }
        for item in tempstack.into_iter().rev() {
            part2crates[dest].push(item);
        }
    }

    print!("Part 1: ");
    for mut c in crates.into_iter().skip(1) {
        print!("{}", c.pop().unwrap());
    }
    print!("\nPart 2: ");
    for mut c in part2crates.into_iter().skip(1) {
        print!("{}", c.pop().unwrap());
    }
}
