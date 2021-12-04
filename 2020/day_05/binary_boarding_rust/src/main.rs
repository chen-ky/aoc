use std::env;
use std::fs;

fn seat_row(seat: &str) -> u32 {
    let mut result = 0;
    let mut power = 7; // Length of FB
    for character in seat.chars() {
        if character == 'B' {
            power -= 1;
            result += 0b1 << power;
        } else if character == 'F' {
            power -= 1;
        }
    }
    result
}

fn seat_column(seat: &str) -> u32 {
    let mut result = 0;
    let mut power = 3; // Length of LR
    for character in seat.chars() {
        if character == 'R' {
            power -= 1;
            result += 0b1 << power;
        } else if character == 'L' {
            power -= 1;
        }
    }
    result
}

fn unique_seat_id(row: u32, column: u32) -> u32 {
    (row * 8) + column
}

fn find_empty_seat(file_content: &String) -> u32 {
    let mut occupied_seats: Vec<u32> = Vec::new();
    for line in file_content.lines() {
        occupied_seats.push(unique_seat_id(seat_row(line), seat_column(line)));
    }
    occupied_seats.sort();
    let mut i = 0;
    while i < occupied_seats.len() - 1 {
        if occupied_seats[i] + 1 == occupied_seats[i + 1] {
            i += 1;
            continue;
        } else {
            return occupied_seats[i] + 1;
        }
    }
    0
}

fn highest_seat_id(file_content: &String) -> u32 {
    let mut result = 0;
    for line in file_content.lines() {
        let id = unique_seat_id(seat_row(line), seat_column(line));
        if id > result {
            result = id;
        }
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file path.");
        std::process::exit(1);
    }

    let file_content = fs::read_to_string(&args[1]).expect("Cannot read file.");
    println!(
        "Part 1 (Highest seat ID): {}",
        highest_seat_id(&file_content)
    );
    println!(
        "Part 2 (Empty seat ID)  : {}",
        find_empty_seat(&file_content)
    );
}
