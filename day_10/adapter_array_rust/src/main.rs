use std::env;
use std::fs;

// All functions assume Vector is sorted.

fn parse_file(file_path: &String) -> Vec<u32> {
    let mut file_content: Vec<u32> = Vec::new();
    for num in fs::read_to_string(file_path).unwrap().split_whitespace() {
        file_content.push(num.parse().unwrap());
    }
    file_content.sort();
    file_content.push(file_content[file_content.len() - 1] + 3); // Device adapter
    file_content
}

// Works on small example, takes too long to run on actual input
fn possible_combinations(file_content: &Vec<u32>, start: u32, end: u32) -> u128 {
    // println!("Start: {}, End: {}", start, end);
    let mut compatible_adapter: Vec<u32> = Vec::new();
    for num in file_content {
        if *num > start && *num <= start + 3 {
            compatible_adapter.push(*num);
        }
    }

    let mut combinations: u128;
    if compatible_adapter.len() == 1 {
        combinations = 1;
        if compatible_adapter[0] == end {
            // Base case
            return combinations;
        } else {
            combinations *= possible_combinations(&file_content, compatible_adapter[0], end);
        }
    } else if compatible_adapter.len() > 1 {
        combinations = 0;
        for num in compatible_adapter {
            combinations += possible_combinations(&file_content, num, end);
        }
    } else {
        panic!("Invalid input?");
    }
    // println!("{}", combinations);
    combinations
}

fn difference_count(file_content: &Vec<u32>) -> [u32; 3] {
    let mut count: [u32; 3] = [0; 3];
    let mut current_adapter = 0;
    let mut i = 0;
    while i < file_content.len() {
        match file_content[i] - current_adapter {
            1 => count[0] += 1,
            2 => count[1] += 1,
            3 => count[2] += 1,
            _ => panic!(),
        }
        current_adapter = file_content[i];
        i += 1
    }

    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide input file path.");
        std::process::exit(1);
    }
    let file_content = parse_file(&args[1]);

    let part1_result = difference_count(&file_content);
    println!(
        "Part 1 ({} * {}): {}",
        part1_result[0],
        part1_result[2],
        part1_result[0] * part1_result[2]
    );
    println!(
        "All possible adapter combinations (Part 2): {}",
        possible_combinations(&file_content, 0, file_content[file_content.len() - 1])
    );
}
