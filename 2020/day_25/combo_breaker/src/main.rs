use std::{env, fs};

fn find_loop_size(key: u64, subject_number: u64) -> u64 {
    let mut i = 0;
    let mut start_number = 1;
    loop {
        start_number = transform(start_number, subject_number);
        i += 1;

        if start_number == key {
            break;
        }
    }
    i
}

fn get_encryption_key(public_key: u64, loop_size: u64) -> u64 {
    transform_n_times(public_key, loop_size)
}

fn transform_n_times(subject_number: u64, loop_size: u64) -> u64 {
    let mut i = 0;
    let mut start_number = 1;
    while i < loop_size {
        start_number = transform(start_number, subject_number);
        i += 1;
    }
    start_number
}

fn transform(start_number: u64, subject_number: u64) -> u64 {
    (start_number * subject_number) % 20201227
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide input file path.");
        std::process::exit(1);
    }

    let file_content = fs::read_to_string(&args[1]).expect("Error reading input file.");

    let public_keys: Vec<&str> = file_content.lines().collect();
    let public_keys = (
        public_keys[0].parse::<u64>().unwrap(),
        public_keys[1].parse::<u64>().unwrap(),
    );
    let loop_sizes = (
        find_loop_size(public_keys.0, 7),
        find_loop_size(public_keys.1, 7),
    );
    println!(
        "Encryption key: {}",
        get_encryption_key(public_keys.1, loop_sizes.0)
    );
}
