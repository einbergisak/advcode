use std::{cell::Cell, fs};

#[derive(Clone, Copy)]
struct Tree {
    size: isize,
    visible: bool,
    score: usize,
}

fn main() {
    let binding = fs::read_to_string("input.txt").unwrap();
    let input = binding.lines().collect::<Vec<&str>>();

    let rows = input.len();
    let cols = input.first().unwrap().len();
    let mut trees = input
        .into_iter()
        .flat_map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .map(|num| Tree {
                    size: num,
                    visible: false,
                    score: 1,
                })
        })
        .collect::<Vec<Tree>>();
    let mut trees2 = trees.clone();
    let largest = Cell::from(-1);

    let mut set_scores = |index: usize| {
        let row = index / cols;
        let col = index % cols;
        let mut update = |range: &mut dyn Iterator<Item = usize>| {
            let mut num_smaller = 1;
            for r in range {
                if trees2[r].size >= trees2[index].size {
                    trees2[index].score *= num_smaller;
                    return;
                }
                num_smaller += 1;
            }
            trees2[index].score *= num_smaller - 1;
        };
        update(&mut ((index + 1)..((row + 1) * cols)));
        update(&mut ((row * cols)..index).rev());
        update(&mut (0..row).map(move |r| col + r * cols).rev());
        update(&mut ((row + 1)..rows).map(move |r| col + r * cols));
    };

    let mut compare_tree = |index: usize| {
        let tree = &mut trees[index];
        if tree.size > largest.get() {
            tree.visible = true;
            largest.set(tree.size);
        }
    };

    for row in 0..rows {
        largest.set(-1);
        for col in 0..cols {
            set_scores(row * cols + col);
            compare_tree(row * cols + col);
        }
        largest.set(-1);
        for col in (0..cols).rev() {
            compare_tree(row * cols + col);
        }
    }
    for col in 0..cols {
        largest.set(-1);
        for row in 0..rows {
            compare_tree(row * cols + col);
        }
        largest.set(-1);
        for row in (0..rows).rev() {
            compare_tree(row * cols + col);
        }
    }

    let num_visible = trees.iter().filter(|tree| tree.visible).count();
    let best_score = trees2
        .into_iter()
        .max_by_key(|tree| tree.score)
        .unwrap()
        .score;
    println!(
        "Number of trees visible: {}. Best score: {}",
        num_visible, best_score
    );
}
