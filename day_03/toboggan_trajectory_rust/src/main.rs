use std::env;
use std::fs;

fn trees_encoutered(file_content: &String, x_step: usize, y_step: usize) -> u64 {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut tree_count: u64 = 0;
    let file_content: Vec<&str> = file_content.lines().collect();
    while y < file_content.len() {
        let line: Vec<char> = file_content[y].chars().collect();
        if line[x % line.len()] == '#' {
            tree_count += 1;
        }
        x += x_step;
        y += y_step;
    }
    tree_count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file path.");
        std::process::exit(1);
    }

    let file_content = fs::read_to_string(&args[1]).expect("Error reading file.");

    println!(
        "Trees encountered (Part 1): {}",
        trees_encoutered(&file_content, 3, 1)
    );

    println!("-----------------------------------------");

    let part2_trees = [
        trees_encoutered(&file_content, 1, 1),
        trees_encoutered(&file_content, 3, 1),
        trees_encoutered(&file_content, 5, 1),
        trees_encoutered(&file_content, 7, 1),
        trees_encoutered(&file_content, 1, 2),
    ];

    println!("Trees encountered (Part 2, (1, 1)): {}", part2_trees[0]);
    println!("Trees encountered (Part 2, (3, 1)): {}", part2_trees[1]);
    println!("Trees encountered (Part 2, (5, 1)): {}", part2_trees[2]);
    println!("Trees encountered (Part 2, (7, 1)): {}", part2_trees[3]);
    println!("Trees encountered (Part 2, (1, 2)): {}", part2_trees[4]);
    let mut result: u64 = 1;
    for count in &part2_trees {
        result *= count;
    }
    println!("Part 2 result: {}", result);
}
