use std::collections::HashMap;
use std::{env, fs};

fn replace_x_address(mask: &str, addresses: Vec<u64>) -> Vec<u64> {
    let mut result_addresses = addresses;
    for (char_index, character) in mask.chars().enumerate() {
        if character == 'X' {
            let mut i = 0;
            let mut new_addresses = Vec::new();
            while i < result_addresses.len() {
                new_addresses.push(result_addresses[i] ^ (0b1 << (35 - char_index))); // Flip bits where X is present
                i += 1;
            }
            for new_address in new_addresses.iter() {
                result_addresses.push(*new_address);
            }
        }
    }
    result_addresses
}

fn parse_mask(mask: &str, version: u8) -> (u64, u64) {
    if mask.len() > 36 {
        panic!("Cannot parse mask: String length greater than 36.");
    }
    let mut copy_mask = 0;
    let mut replacement = 0;
    let mut power = 35;
    for bit in mask.chars() {
        match version {
            1 => match bit {
                'X' => copy_mask += 0b1 << power, // if X, no changes needed, therefore copy
                '1' => replacement += 0b1 << power,
                '0' => {}
                _ => panic!("Invalid character in mask. {}", mask),
            },
            2 => match bit {
                'X' => {}
                '1' => replacement += 0b1 << power,
                '0' => copy_mask += 0b1 << power,
                _ => panic!("Invalid character in mask. {}", mask),
            },
            _ => panic!("Invalid version number. [1,2]"),
        }
        power -= 1;
    }
    (copy_mask, replacement)
}

fn apply_mask(number: u64, copy_mask: u64, replacement: u64) -> u64 {
    // Copy bits not being replace and add replacement
    // 0b111 & 0b001 = 0b001
    // 0b001 + 0b10X = 0b101
    (number & copy_mask) + replacement
}

fn overflow_36_bits(number: u64) -> bool {
    let mask = 0xFF_FF_FF_F0_00_00_00_00;
    if number & mask > 0 {
        return true;
    }
    false
}

fn run(file_content: &str, version: u8) -> HashMap<u64, u64> {
    let mut memory = HashMap::new();
    let mut raw_mask = "";
    let mut copy_mask = 0;
    let mut replacement = 0;
    for line in file_content.lines() {
        if line.starts_with("mask") {
            raw_mask = line.strip_prefix("mask = ").unwrap();
            let masks = parse_mask(raw_mask, version);
            copy_mask = masks.0;
            replacement = masks.1;
        } else if line.starts_with("mem") {
            let mem = line.replace(" ", "");
            let mem = mem.splitn(2, '=').collect::<Vec<&str>>();
            let mem_location: u64 = mem[0]
                .strip_prefix("mem[")
                .unwrap()
                .strip_suffix("]")
                .unwrap()
                .parse()
                .unwrap();
            let value: u64 = mem[1].parse().unwrap();
            match version {
                1 => {
                    let result = apply_mask(value, copy_mask, replacement);
                    if overflow_36_bits(result) {
                        panic!("Overflow.");
                    }
                    memory.insert(mem_location, result);
                }
                2 => {
                    let result = apply_mask(mem_location, copy_mask, replacement);
                    let addresses = replace_x_address(raw_mask, vec![result]);
                    for address in addresses {
                        if overflow_36_bits(address) {
                            panic!("Overflow.");
                        }
                        memory.insert(address, value);
                    }
                }
                _ => panic!("Invalid version. [1,2]"),
            }
        } else {
            panic!("Invalid instruction.\n{}", line);
        }
    }
    memory
}

fn memory_value_sum(memory_map: &HashMap<u64, u64>) -> u128 {
    let mut result = 0;
    for value in memory_map.values() {
        result += *value as u128;
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide input file path.");
        std::process::exit(1);
    }

    let file_content = fs::read_to_string(&args[1]).expect("Unable to read input file.");

    let part1_memory = run(&file_content, 1);
    println!("Part 1 sum: {}", memory_value_sum(&part1_memory));
    let part2_memory = run(&file_content, 2);
    println!("Part 2 sum: {}", memory_value_sum(&part2_memory));
}
