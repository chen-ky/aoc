use std::fs;

fn file_parser(file_path: &str) -> Vec<u32> {
    // Read file
    let file_content = fs::read_to_string(file_path)
        .expect("Error reading file");

    // Split and parse string to u32
    let file_content = file_content.split_whitespace();
    let mut data = Vec::new();
    for content in file_content {
        let num: u32 = match content.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        data.push(num);
    }

    return data
}

fn solve_part_1(data: &Vec<u32>) -> u32 {
    let mut i = 0;
    while i < data.len() {
        let num1 = data[i];
        let mut j = 0;
        while j < data.len() {
            // Find solution
            let num2 = data[j];
            if 2020 == (num1 + num2) {
                return num1 * num2
            }
            j += 1;
        }
        i += 1;
    }
    return 0
}

fn solve_part_2(data: &Vec<u32>) -> u32 {
    let mut i = 0;
    while i < data.len() {
        let num1 = data[i];
        let mut j = 0;
        while j < data.len() {
            let num2 = data[j];
            let mut k = 0;
            while k < data.len() {
                // Find solution
                let num3 = data[k];
                if 2020 == (num1 + num2 + num3) {
                    return num1 * num2 * num3
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    return 0
}

fn main() {
    let data = file_parser("resources/input"); 
    
    println!("Part 1 solution: {}", solve_part_1(&data));
    println!("Part 2 solution: {}", solve_part_2(&data));
}
