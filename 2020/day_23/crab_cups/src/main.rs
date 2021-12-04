// Fix this
use std::collections::LinkedList;

fn game_move(cup_sequence: &mut Vec<u32>, current_cup_index: usize) {
    let current_cup = cup_sequence[current_cup_index];
    let mut picked_up_cup: Vec<u32> = Vec::new();

    // Picked up cup
    let mut remove_index = current_cup_index + 1;
    loop {
        // picked_up_cup.push(cup_sequence.remove((current_cup_index + 1) % cup_sequence.len()));
        if remove_index >= cup_sequence.len() {
            remove_index = 0;
        }
        picked_up_cup.push(cup_sequence.remove(remove_index));
        if picked_up_cup.len() == 3 {
            break;
        }
    }
    
    // Destination cup
    // Determine max and min value for cup sequence
    let mut max_value = cup_sequence[0];
    let mut min_value = cup_sequence[0];
    let mut i = 0;
    while i < cup_sequence.len() {
        let cup = cup_sequence[i];
        if cup > max_value {
            max_value = cup;
        } else if cup < min_value {
            min_value = cup;
        }
        i += 1;
    }
    let mut destination_cup_index: usize = 0;
    let mut destination_cup = current_cup;
    loop {
        destination_cup -= 1;
        if destination_cup < min_value {
            destination_cup = max_value;
        }
        if cup_sequence.contains(&destination_cup) {
            let mut i = 0;
            while i < cup_sequence.len() {
                let cup = cup_sequence[i];
                if cup == destination_cup {
                    break;
                }
                destination_cup_index += 1;
                i += 1;
            }
            break;
        }
    }

    picked_up_cup.reverse();
    for cup in &picked_up_cup {
        cup_sequence.insert(destination_cup_index + 1 , *cup);
    }

    // Index adjustment, make sure current cup is at same index
    let mut index = 0;
    while index < cup_sequence.len() {
        let cup = cup_sequence[index];
        if cup == current_cup {
            break;
        }
        index += 1;
    }
    let mut offset: isize = (current_cup_index as isize) - (index as isize);
    while offset != 0 {
        if offset > 0 {
            let cup = cup_sequence.pop().unwrap();
            cup_sequence.insert(0, cup);
            offset -= 1;
        } else if offset < 0 {
            let value = cup_sequence.remove(0);
            cup_sequence.push(value);
            offset += 1;
        }
    }
    picked_up_cup.reverse();
    // println!("{:?}", picked_up_cup);
}

fn play(input: &mut Vec<u32>, rounds: usize) {
    for num in 0..rounds {
        // println!("{:?}", input);
        game_move(input, num % input.len());
    }
}

fn main() {
    // let mut input = vec!(3, 8, 9, 1, 2, 5, 4, 6, 7);
    let mut input = vec!(1, 5, 8, 9, 3, 7, 4, 6, 2);
    
    play(&mut input, 100);
    println!("{:?}", input);

    let mut input = vec!(3, 8, 9, 1, 2, 5, 4, 6, 7);
    // let mut input = vec!(1, 5, 8, 9, 3, 7, 4, 6, 2);
    for num in 10..10000001 {
        input.push(num);
    }
    play(&mut input, 10000000);
    println!("{:?}", input);
}