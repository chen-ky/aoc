use std::env;
use std::fs;

/** Return value of error */
fn find_encoding_error(file_content: &Vec<u64>, preamble_size: usize) -> u64 {
    if file_content.len() < preamble_size + 1 {
        return 0;
    }

    let mut range = (0, preamble_size - 1);
    let mut result = 0;
    while range.1 + 1 < file_content.len() {
        let to_match = file_content[range.1 + 1];
        let mut matched = false;
        for i in range.0..range.1 + 1 {
            // Compute everything below current index i
            for j in i + 1..range.1 + 1 {
                // println!("{}, {}", file_content[i], file_content[j]);
                if file_content[i] + file_content[j] == to_match {
                    matched = true;
                    break;
                }
            }
            if matched {
                break;
            }
        }

        if !matched {
            result = to_match;
            break;
        }
        range.0 += 1;
        range.1 += 1;
    }
    result
}

fn find_contiguous_set(file_content: &Vec<u64>, to_find: u64) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    // Find index of `to_find` number, seach will terminate before `to_find` index.
    // All sum of number after index is bigger.
    let mut to_find_index: usize = 0;
    let mut i = 0;
    for num in file_content {
        if *num == to_find {
            to_find_index = i;
            break;
        }
        i += 1
    }

    let mut start_index = 0;
    let mut current_index = start_index;
    let mut sum = 0;
    while current_index < to_find_index {
        sum += file_content[current_index];

        while sum > to_find {
            sum -= file_content[start_index]; // Minus first element in the sum
            start_index += 1;
        }

        if sum == to_find {
            for index in start_index..current_index + 1 {
                result.push(file_content[index]);
            }
            break;
        }
        current_index += 1;
    }
    result
}

fn min_max(part2_result: &Vec<u64>) -> u64 {
    if part2_result.len() < 2 {
        return 0;
    }
    let mut min = part2_result[0];
    let mut max = part2_result[0];
    for num in part2_result {
        if *num > max {
            max = *num
        } else if *num < min {
            min = *num
        }
    }
    min + max
}

fn parse_file(file_content: String) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    for line in file_content.split_whitespace() {
        result.push(line.parse().unwrap());
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Please provide input file path and preamble length.");
        std::process::exit(1);
    }

    let file_content = fs::read_to_string(&args[1]).expect("Cannot read file.");
    let file_content = parse_file(file_content);

    let part1_result = find_encoding_error(&file_content, args[2].parse().unwrap());
    println!("First number with error: {}", part1_result);

    println!("");
    let part2_result = find_contiguous_set(&file_content, part1_result);
    println!("Part 2 set: {:?}", part2_result);
    println!("Sum of min, max value: {}", min_max(&part2_result));
}
