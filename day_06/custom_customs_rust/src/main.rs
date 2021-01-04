use std::env;
use std::fs;

fn parse_file(file_content: &str) -> Vec<Vec<Vec<char>>> {
    let mut answers: Vec<Vec<Vec<char>>> = Vec::new();
    
    let mut group: Vec<Vec<char>> = Vec::new();
    for line in file_content.lines() {
        if line != "" {
            let mut person_answer = Vec::new();
            for character in line.chars() {
                person_answer.push(character);
            }
            group.push(person_answer);
        } else {
            if !group.is_empty() {
                answers.push(group);
                group = Vec::new();
            }
        }
    }
    if !group.is_empty() {
        answers.push(group);
    }
    answers
}

fn remove_duplicate_answer(group: &Vec<Vec<char>>) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    for people in group.iter() {
        for answer in people.iter() {
            if !result.contains(answer) {
                result.push(*answer);
            }
        }
    }
    result
}

fn everyone_answered_yes(group: &Vec<Vec<char>>) -> Vec<char> {
    let to_find = remove_duplicate_answer(group);
    let mut result: Vec<char> = Vec::new();
    for question in to_find {
        let mut all_answered_yes = true;
        for people in group.iter() {
            if !people.contains(&question) {
                all_answered_yes = false;
                break;
            }
        }
        if !all_answered_yes {
            continue;
        } else {
            result.push(question);
        }
    }
    result
}

fn part_1(answers: &Vec<Vec<Vec<char>>>) -> usize {
    let mut counter = 0;
    for group in answers.iter() {
        counter += remove_duplicate_answer(group).len();
    }
    counter
}
fn part_2(answers: &Vec<Vec<Vec<char>>>) -> usize {
    let mut counter = 0;
    for group in answers.iter() {
        counter += everyone_answered_yes(group).len();
    }
    counter
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide input file path.");
        std::process::exit(1);
    }

    let file_content = fs::read_to_string(&args[1]).expect("Error reading file.");
    let answers = parse_file(&file_content);

    println!("Part 1 count: {}", part_1(&answers));
    println!("Part 2 count: {}", part_2(&answers));
}
