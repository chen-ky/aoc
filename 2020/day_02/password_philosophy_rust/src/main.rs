use std::env;
use std::fs;

fn read_file(file_path: &String) -> String {
    let file_content = fs::read_to_string(file_path).expect("Error reading file.");
    file_content
}

fn parse_line(line: &str) -> (u32, u32, char, &str) {
    let line: Vec<&str> = line.split_whitespace().collect();

    let num: Vec<&str> = line[0].split('-').collect();
    let num0: u32 = num[0].parse().unwrap();
    let num1: u32 = num[1].parse().unwrap();
    let character = line[1].chars().collect::<Vec<char>>()[0];
    (num0, num1, character, line[2])
}

fn count_valid_password_part_1(file_content: &String) -> u32 {
    let mut valid_pass_count = 0;
    for line in file_content.lines() {
        let (min, max, character, password) = parse_line(line);
        let mut char_counter = 0;
        for password_char in password.chars() {
            if password_char == character {
                char_counter += 1;
            }
        }
        if char_counter < min || char_counter > max {
            continue;
        } else {
            valid_pass_count += 1;
        }
    }
    valid_pass_count
}

fn count_valid_password_part_2(file_content: &String) -> u32 {
    let mut valid_pass_count = 0;
    for line in file_content.lines() {
        let (position0, position1, character, password) = parse_line(line);
        let password_chars: Vec<char> = password.chars().collect();
        let mut char_at_position = (false, false);
        if position0 - 1 < password_chars.len() as u32 {
            char_at_position.0 = password_chars[position0 as usize - 1] == character;
        }
        if position1 - 1 < password_chars.len() as u32 {
            char_at_position.1 = password_chars[position1 as usize - 1] == character;
        }
        if char_at_position.0 ^ char_at_position.1 {
            valid_pass_count += 1;
        }
    }
    valid_pass_count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file path.");
        std::process::exit(1);
    }

    let file_content = read_file(&args[1]);
    println!(
        "Part 1 valid password count: {}",
        count_valid_password_part_1(&file_content)
    );
    println!(
        "Part 2 valid password count: {}",
        count_valid_password_part_2(&file_content)
    );
}
