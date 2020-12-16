use std::env;
use std::fs;

fn fix_infinite_loop(instructions: &Vec<&str>) -> (usize, isize) {
    let mut pc: usize = 0;
    let mut accumulator: isize = 0;
    let mut tried: Vec<usize> = Vec::new(); // Instruction flipped

    loop {
        if pc >= instructions.len() {
            break;
        }
        let mut executed_lines: Vec<usize> = Vec::new();
        let mut changed_instruction = false;
        pc = 0;
        accumulator = 0;
        loop {
            if executed_lines.contains(&pc) || pc >= instructions.len() {
                break;
            }
            executed_lines.push(pc);
            let mut current_instruction: Vec<&str> = instructions[pc].split_whitespace().collect();
            if current_instruction[0] == "nop" && !tried.contains(&pc) && !changed_instruction {
                current_instruction[0] = "jmp";
                changed_instruction = true;
                tried.push(pc);
            } else if current_instruction[0] == "jmp"
                && !tried.contains(&pc)
                && !changed_instruction
            {
                current_instruction[0] = "nop";
                changed_instruction = true;
                tried.push(pc);
            }
            let result = emulate(
                pc,
                accumulator,
                &current_instruction[0],
                &current_instruction[1],
            );
            pc = result.0;
            accumulator = result.1;
        }
    }
    (pc, accumulator)
}

fn find_infinite_loop(instructions: &Vec<&str>) -> (usize, isize, bool) {
    let mut pc: usize = 0;
    let mut accumulator: isize = 0;
    let mut executed_lines: Vec<usize> = Vec::new();

    loop {
        if executed_lines.contains(&pc) {
            break;
        } else if pc >= instructions.len() {
            return (pc, accumulator, false);
        }
        executed_lines.push(pc);
        let current_instruction: Vec<&str> = instructions[pc].split_whitespace().collect();
        let result = emulate(
            pc,
            accumulator,
            &current_instruction[0],
            &current_instruction[1],
        );
        pc = result.0;
        accumulator = result.1;
    }
    (pc, accumulator, true)
}

fn emulate(pc: usize, accumulator: isize, operation: &str, operand: &str) -> (usize, isize) {
    let mut pc: usize = pc;
    let mut accumulator: isize = accumulator;

    if operation == "nop" {
        pc += 1;
    } else if operation == "acc" {
        let number: isize = operand.parse().unwrap();
        accumulator += number;
        pc += 1;
    } else if operation == "jmp" {
        if operand.starts_with('-') {
            let number: usize = operand.trim_start_matches('-').parse().unwrap();
            pc -= number;
        } else {
            let number: usize = operand.parse().unwrap();
            pc += number;
        }
    }
    (pc, accumulator)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file path.");
        std::process::exit(1);
    }

    let file_content = fs::read_to_string(&args[1]).expect("Cannot read file.");
    let mut instructions: Vec<&str> = Vec::new();
    for line in file_content.lines() {
        instructions.push(line);
    }

    println!(
        "Part 1 accumulator: {}",
        find_infinite_loop(&instructions).1
    );
    println!(
        "Part 2 accumulator: {}", 
        fix_infinite_loop(&instructions).1
    );
}
