use std::collections::HashMap;
use std::{env, fs, process};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Field {
    name: String,
    rules: Vec<(u32, u32)>,
}

fn parse_file(file_path: &str) -> (Vec<Field>, Vec<u32>, Vec<Vec<u32>>) {
    let mut fields = vec![];
    let mut own_ticket = vec![];
    let mut nearby_tickets = vec![];
    let mut parsing_rule = true;
    let mut parsing_own_ticket = false;
    for line in fs::read_to_string(file_path)
        .expect("Unable to read file.")
        .lines()
    {
        if line == "" {
            continue;
        } else if line.starts_with("your ticket:") {
            parsing_own_ticket = true;
            parsing_rule = false;
            continue;
        } else if line.starts_with("nearby tickets:") {
            parsing_own_ticket = false;
            parsing_rule = false;
            continue;
        }

        if parsing_rule {
            let line_content: Vec<&str> = line.splitn(2, ": ").collect();
            let rule_line: Vec<&str> = line_content[1].split(" or ").collect();
            let mut rules = vec![];
            for rule in rule_line.iter() {
                let bound: Vec<&str> = rule.split('-').collect();
                rules.push((bound[0].parse().unwrap(), bound[1].parse().unwrap()));
            }
            fields.push(Field {
                name: line_content[0].to_string(),
                rules,
            })
        } else {
            let line_content: Vec<&str> = line.split(',').collect();
            let mut ticket = vec![];
            for number in line_content.iter() {
                if parsing_own_ticket {
                    own_ticket.push(number.parse().unwrap());
                } else {
                    ticket.push(number.parse().unwrap());
                }
            }
            if !parsing_own_ticket {
                nearby_tickets.push(ticket);
            }
        }
    }
    (fields, own_ticket, nearby_tickets)
}

fn satisfy_rule(field: &Field, number: u32) -> bool {
    let mut matched = false;
    for (lower_bound, upper_bound) in field.rules.iter() {
        if number >= *lower_bound && number <= *upper_bound {
            matched = true;
            break;
        }
    }
    matched
}

fn is_valid_ticket(fields: &Vec<Field>, ticket: &Vec<u32>) -> Result<(), u32> {
    for number in ticket.iter() {
        let mut matched = false;
        for field in fields.iter() {
            if satisfy_rule(field, *number) {
                matched = true;
                break;
            }
        }
        if !matched {
            return Err(*number);
        }
    }
    Ok(())
}

fn get_invalid_field_sum(fields: &Vec<Field>, tickets: &Vec<Vec<u32>>) -> u32 {
    let mut invalid_sum = 0;
    for ticket in tickets.iter() {
        match is_valid_ticket(fields, ticket) {
            Ok(()) => {}
            Err(n) => invalid_sum += n,
        }
    }
    invalid_sum
}

fn match_fields(
    fields: &Vec<Field>,
    own_ticket: &Vec<u32>,
    nearby_tickets: &Vec<Vec<u32>>,
) -> HashMap<Field, usize> {
    let mut matched_fields = HashMap::new();
    let mut field_valid_for_columns: HashMap<Field, Vec<usize>> = HashMap::new();
    let mut valid_tickets = vec![];

    // Remove invalid tickets
    for ticket in nearby_tickets.iter() {
        match is_valid_ticket(fields, ticket) {
            Ok(()) => valid_tickets.push(ticket),
            Err(_n) => {}
        }
    }

    // Find all possible match for each field
    for i in 0..own_ticket.len() {
        for field in fields.iter() {
            let mut all_satisfied = satisfy_rule(field, *own_ticket.get(i).unwrap());
            let mut columns = vec![];
            if field_valid_for_columns.contains_key(field) {
                columns = field_valid_for_columns.get(field).unwrap().to_vec();
            }

            for ticket in valid_tickets.iter() {
                if !satisfy_rule(field, *ticket.get(i).unwrap()) || !all_satisfied {
                    all_satisfied = false;
                    break;
                }
            }
            if all_satisfied {
                columns.push(i);
            }
            field_valid_for_columns.insert(field.clone(), columns);
        }
    }

    // Find unique match for each field
    let mut field_columns_length = vec![];
    let mut matched = vec![];
    for columns in field_valid_for_columns.values() {
        field_columns_length.push(columns.len());
    }
    field_columns_length.sort();

    while field_columns_length.len() > 0 {
        let find_length = field_columns_length.remove(0);
        for key in field_valid_for_columns.keys() {
            let columns = field_valid_for_columns.get(key).unwrap();
            if columns.len() == find_length && !matched.contains(key) {
                for index in columns.iter() {
                    if !matched_fields
                        .values()
                        .collect::<Vec<&usize>>()
                        .contains(&index)
                    {
                        matched_fields.insert(key.clone(), *index);
                        matched.push(key.clone());
                    }
                }
            }
        }
    }
    matched_fields
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide input file path.");
        process::exit(1);
    }

    let (fields, own_ticket, nearby_tickets) = parse_file(&args[1]);

    println!(
        "Sum of invalid ticket fields (Part 1): {}",
        get_invalid_field_sum(&fields, &nearby_tickets)
    );

    println!();

    println!("My ticket: ");
    let matched_fields = match_fields(&fields, &own_ticket, &nearby_tickets);
    let mut product: u64 = 1;
    for (field, index) in matched_fields {
        let field_value = own_ticket.get(index).unwrap();
        println!("{}: {}", field.name, field_value);
        if field.name.contains("departure") {
            product *= *field_value as u64;
        }
    }
    println!("Part 2 result: {}", product);
}
