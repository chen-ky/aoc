use std::env;
use std::fs;

fn parse_file(file_path: &str) -> Vec<Vec<char>> {
    let mut seating_layout: Vec<Vec<char>> = Vec::new();
    for line in fs::read_to_string(file_path)
        .expect("Error reading file.")
        .lines()
    {
        let mut row: Vec<char> = Vec::new();
        for seat in line.chars() {
            row.push(seat);
        }
        seating_layout.push(row);
    }
    seating_layout
}

fn simulate_seating(seats: Vec<Vec<char>>, part: u8) -> Vec<Vec<char>> {
    let mut previous_iteration = seats;
    loop {
        let mut next_itereation = Vec::new();
        for row in previous_iteration.iter() {
            next_itereation.push(row.clone());
        }

        let mut has_changed = false;
        for (row_index, row) in previous_iteration.iter().enumerate() {
            for (column_index, seat) in row.iter().enumerate() {
                let will_change = match part {
                    1 => to_change_part1(&previous_iteration, row_index, column_index),
                    2 => to_change_part2(&previous_iteration, row_index, column_index),
                    _ => panic!("Invalid part. Valid: [1, 2]"),
                };
                if will_change {
                    has_changed = true;
                    next_itereation[row_index][column_index] = match seat {
                        'L' => '#',
                        '#' => 'L',
                        _ => panic!("No match"),
                    };
                };
            }
        }
        if !has_changed {
            return next_itereation;
        }
        previous_iteration = next_itereation;
    }
}

fn adjacent_seats_part_1(seats: &[Vec<char>], row: usize, col: usize) -> Vec<char> {
    let mut adjacent_seats = Vec::new();
    let access_pattern = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut check_row = row as isize;
    let mut check_col = col as isize;
    for check_pattern in access_pattern.iter() {
        check_row += check_pattern.0;
        check_col += check_pattern.1;

        if check_col >= 0 && check_row >= 0 {
            let check_col = check_col as usize;
            let check_row = check_row as usize;
            if check_row < seats.len() {
                if check_col < seats[check_row].len() {
                    adjacent_seats.push(seats[check_row][check_col]);
                }
            }
        }
        check_row = row as isize;
        check_col = col as isize;
    }
    adjacent_seats
}

fn adjacent_seats_part_2(seats: &[Vec<char>], row: usize, col: usize) -> Vec<char> {
    let mut adjacent_seats = Vec::new();
    let access_pattern = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut check_row = row as isize;
    let mut check_col = col as isize;
    for check_pattern in access_pattern.iter() {
        loop {
            check_row += check_pattern.0;
            check_col += check_pattern.1;
            if check_col >= 0 && check_row >= 0 {
                let check_col = check_col as usize;
                let check_row = check_row as usize;
                if check_row < seats.len() {
                    if check_col < seats[check_row].len() {
                        if seats[check_row][check_col] == '#' || seats[check_row][check_col] == 'L'
                        {
                            adjacent_seats.push(seats[check_row][check_col]);
                            break;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        check_row = row as isize;
        check_col = col as isize;
    }
    adjacent_seats
}

fn to_change_part1(seats: &[Vec<char>], row: usize, col: usize) -> bool {
    match seats[row][col] {
        '.' => false,
        'L' => {
            for seat in adjacent_seats_part_1(seats, row, col) {
                if seat == '#' {
                    return false;
                }
            }
            true
        }
        '#' => {
            let mut counter = 0;
            for seat in adjacent_seats_part_1(seats, row, col) {
                if seat == '#' {
                    counter += 1;
                }

                if counter >= 4 {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}

fn to_change_part2(seats: &[Vec<char>], row: usize, col: usize) -> bool {
    match seats[row][col] {
        '.' => false,
        'L' => {
            for seat in adjacent_seats_part_2(seats, row, col) {
                if seat == '#' {
                    return false;
                }
            }
            true
        }
        '#' => {
            let mut counter = 0;
            for seat in adjacent_seats_part_2(seats, row, col) {
                if seat == '#' {
                    counter += 1;
                }

                if counter >= 5 {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}

fn print_seats(seats: &[Vec<char>]) {
    for row in seats {
        for seat in row {
            print!("{}", seat);
        }
        println!();
    }
}

fn count_occupied_seats(seats: &[Vec<char>]) -> u32 {
    let mut counter: u32 = 0;
    for row in seats {
        for seat in row {
            if *seat == '#' {
                counter += 1;
            }
        }
    }
    counter
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide input file path.");
        std::process::exit(1);
    }
    let seating_layout: Vec<Vec<char>> = parse_file(&args[1]);

    let part1_final_seats = simulate_seating(seating_layout.clone(), 1);
    // print_seats(&part1_final_seats);
    println!(
        "Final occupied seats count (Part 1): {}",
        count_occupied_seats(&part1_final_seats)
    );

    let part2_final_seats = simulate_seating(seating_layout.clone(), 2);
    // print_seats(&part2_final_seats);
    println!(
        "Final occupied seats count (Part 2): {}",
        count_occupied_seats(&part2_final_seats)
    );
}
