use std::{fs, env};

// Return string from first detected bracket until the matching closing bracket
// Note: This does not do a complete check to make sure the equation have complete matching brackets,
// It simply returns the first valid string inside a bracket
fn unwrap_bracket(equation: &str) -> Result<&str, &str> {
    let mut incomplete_brackets = 0;
    let mut open_brackets = 0;
    let mut slice_range = (0, equation.len());
    for (index, character) in equation.chars().enumerate() {
        match character {
            '(' => {
                if open_brackets == 0 {
                    slice_range.0 = index + 1; // Skip bracket character
                }
                incomplete_brackets += 1;
                open_brackets += 1;
            },
            ')' => {
                incomplete_brackets -= 1;
                if open_brackets > 0 && incomplete_brackets == 0 {
                    slice_range.1 = index;
                    break;
                }
            },
            _ => continue,
        }
    }
    if incomplete_brackets != 0 {
        // println!("{}", equation);
        return Err("Unmatched bracket.");
    }
    Ok(&equation[slice_range.0 .. slice_range.1])
}

fn contain_brackets(equation: &str) -> Result<bool, &str> {
    let mut opening_brackets = 0;
    let mut closing_brackets = 0;
    for character in equation.chars() {
        match character {
            '(' => opening_brackets += 1,
            ')' => closing_brackets += 1,
            _ => (),
        }
        if closing_brackets > opening_brackets {
            return Err("Unmatched bracket.");
        }
    }
    if closing_brackets == opening_brackets {
        if closing_brackets == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    } else {
        return Err("Unmatched bracket.");
    }
}

fn compute(number_0: i128, operator: &char, number_1: i128) -> Result<i128, &str> {
    println!("{} {} {}", number_0, operator, number_1);
    match operator {
        '+' => {println!("{}", number_0 + number_1); Ok(number_0 + number_1)},
        '*' => {println!("{}", number_0 * number_1); Ok(number_0 * number_1)},
        _ => Err("Invalid operator."),
    }
}

fn evaluate(equation: &str) -> Result<i128, &str> {
    match contain_brackets(equation) {
        Ok(contain_bracket) => {
            if contain_bracket {
                let mut left = "";
                let mut operator = ' ';
                let mut right = "";

                if equation.starts_with('(') {
                    left = unwrap_bracket(equation).unwrap();
                    let splitted_equation = equation[1..].strip_prefix(left).unwrap().strip_prefix(')').unwrap().trim().splitn(2, ' ').collect::<Vec<&str>>();
                    if splitted_equation.len() == 2 {
                        operator = splitted_equation[0].chars().collect::<Vec<char>>()[0];
                        right = splitted_equation[1];
                        let number_0 = evaluate(left).unwrap();
                        let number_1 = evaluate(right).unwrap();
        
                        Ok(compute(number_0, &operator, number_1).unwrap())
                    } else {
                        Ok(evaluate(left).unwrap())
                    }

                } else {
                    let splitted_equation = equation.splitn(3, ' ').collect::<Vec<&str>>();
                    left = splitted_equation[0];
                    operator = splitted_equation[1].chars().collect::<Vec<char>>()[0];
                    right = splitted_equation[2];
                    let number_0 = evaluate(left).unwrap();
                    let number_1 = evaluate(right).unwrap();
    
                    Ok(compute(number_0, &operator, number_1).unwrap())
                }
            } else {
                let splitted_equation = equation.split_whitespace().collect::<Vec<&str>>();
                let mut result = splitted_equation[0].parse().unwrap();
                let mut i = 1;
                while i < splitted_equation.len() {
                    let operator = splitted_equation[i].chars().collect::<Vec<char>>()[0];
                    let number_1 = splitted_equation[i + 1].parse().unwrap();
                    result = compute(result, &operator, number_1).unwrap();
                    i += 2;
                }
                Ok(result)
            }
        },
        Err(e) => Err(e),
    }
}
// fn evaluate(equation: &str) -> Result<i128, &str> {
//     println!("{}",equation);
//     std::thread::sleep(std::time::Duration::new(1,0));
//     if equation.parse::<i128>().is_ok() {
//         return Ok(equation.parse().unwrap());
//     }

//     let mut result = 0;
//     let splitted_equation = equation.splitn(3, ' ').collect::<Vec<&str>>();
//     let number_0 = {
//         if splitted_equation[0].starts_with('(') {
//             evaluate(unwrap_bracket(equation).unwrap()).unwrap()
//         } else if splitted_equation[0].parse::<i128>().is_ok() {
//             splitted_equation[0].parse().unwrap()
//         } else {
//             evaluate(splitted_equation[0]).unwrap()
//         }
//     };
//     // println!("{}", number_0);
//     let operator = splitted_equation[1];
//     let number_1 = {
//         if splitted_equation[2].starts_with('(') {
//             evaluate(unwrap_bracket(splitted_equation[2]).unwrap()).unwrap()
//         } else if splitted_equation[2].parse::<i128>().is_ok() {
//             splitted_equation[2].parse().unwrap()
//         } else {
//             evaluate(splitted_equation[2]).unwrap()
//         }
//     };
    
//     Ok(result)
// }

fn main() {
    // println!("{}", unwrap_bracket("").unwrap());
    // println!("{:?}", "(5 * ((7 * 3 + 8)) * 9 * 8 + 8 + (9 + 7 + 5 + 8))".split_whitespace().collect::<Vec<&str>>());
    // println!("{:?}", evaluate("5 * ((7 * 3 + 8) * 9 * 8 + 8 + (9 + 7 + 5 + 8))").unwrap());
    // println!("{}", &"((7 * 3 + 8) * 9 * 8 + 8 + (9 + 7 + 5 + 8))"[9..]);
    // println!("{:?}", evaluate("1 + (2 * 3) + (4 * (5 + 6))").unwrap());
    // println!("{:?}", evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap());
    println!("{:?}", evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap());
}
