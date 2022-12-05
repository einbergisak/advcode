use std::fs;

fn main() {
    let mut crates: Vec<Vec<char>> = Vec::new();
    crates.push(vec![]);
    crates.push(vec!['Z','J','N','W','P','S']);
    crates.push(vec!['G','S','T']);
    crates.push(vec!['V','Q','R','L','H']);
    crates.push(vec!['V','S','T','D']);
    crates.push(vec!['Q','Z','T','D','B','M', 'J']);
    crates.push(vec!['M','W','T','J','D','C','Z','L']);
    crates.push(vec!['L','P','M','W','G','T','J']);
    crates.push(vec!['N','G','M','T','B','F','Q','H']);    
    crates.push(vec!['R','D','G','C','P','B','Q','W']);

    let binding = fs::read_to_string("input.txt").unwrap();
    let mut input = binding
        .split_whitespace()
        .filter_map(|val| val.parse::<usize>().ok());

    let mut part2crates = crates.clone();
    while let (Some(count), Some(source), Some(dest)) = (input.next(), input.next(), input.next()){
        let mut tempstack: Vec<char> = Vec::new();
        for _ in 0..count {
            let val1 = crates[source].pop().unwrap();
            crates[dest].push(val1);
            let val2 = part2crates[source].pop().unwrap();
            tempstack.push(val2);
        }
        for item in tempstack.into_iter().rev(){
            part2crates[dest].push(item);
        }
    } 


    print!("Part 1: ");
    for mut c in crates.into_iter().skip(1) {
        print!("{}",c.pop().unwrap());
    }
    print!("\nPart 2: ");
    for mut c in part2crates.into_iter().skip(1) {
        print!("{}",c.pop().unwrap());
    }
}
