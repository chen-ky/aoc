use std::env;
use std::fs;
use std::collections::HashMap;

fn parse_file(filepath: &String) -> (Vec<u32>, Vec<u32>) {
    let mut player1: Vec<u32> = Vec::new();
    let mut player2: Vec<u32> = Vec::new();
    let file_content = fs::read_to_string(filepath).expect("Invalid file path.");

    let mut player1_done = false;
    for line in file_content.split_terminator("\n") {
        if line == "" {
            continue;
        }
        if line == "Player 1:" {
            player1_done = false;
            continue;
        } else if line == "Player 2:" {
            player1_done = true;
            continue;
        }

        if player1_done {
            player2.push(line.parse().unwrap());
        } else {
            player1.push(line.parse().unwrap());
        }
    }
    (player1, player2)
}

fn get_score(players: &(Vec<u32>, Vec<u32>)) -> Option<u32> {
    let mut result = 0;
    if players.0.len() == 0 {
        let mut multiplier = players.1.len() as u32;
        for card in players.1.iter() {
            result += multiplier * card;
            multiplier -= 1;
        }
        Some(result)
    } else if players.1.len() == 0 {
        let mut multiplier = players.0.len() as u32;
        for card in players.0.iter() {
            result += multiplier * card;
            multiplier -= 1;
        }
        Some(result)
    } else {
        None
    }
}

fn play_combat_round(player_1: &mut Vec<u32>, player_2: &mut Vec<u32>) {
    let player1_card = player_1.remove(0);
    let player2_card = player_2.remove(0);
    if player1_card > player2_card {
        player_1.push(player1_card);
        player_1.push(player2_card);
    } else {
        player_2.push(player2_card);
        player_2.push(player1_card);
    }
}

fn play_recursive_combat_round(mut player_1: Vec<u32>, mut player_2: Vec<u32>, mut recursion_history: HashMap<Vec<u32>, (bool, bool)>) -> (Vec<u32>, Vec<u32>, HashMap<Vec<u32>, (bool, bool)>) {
    // println!("{:?}", (&player_1, &player_2));
    let mut card_history = (*recursion_history.get(&player_1).unwrap_or(&(false, false)), *recursion_history.get(&player_2).unwrap_or(&(false, false)));
    card_history.0.0 = true;
    card_history.1.1 = true;
    recursion_history.insert(player_1.clone(), card_history.0);
    recursion_history.insert(player_2.clone(), card_history.1);
    
    if player_1.len() - 1 >= *player_1.get(0).unwrap() as usize && player_2.len() - 1 >= *player_2.get(0).unwrap() as usize {
        let current_cards = (&player_1.remove(0), &player_2.remove(0));
        // println!("recurse: {:?}", (&player_1, &player_2));
        let result = play(player_1.clone(), player_2.clone(), 2);
        // println!("back");
        if result.0.len() == 0 {
            player_2.push(*current_cards.1);
            player_2.push(*current_cards.0);
        } else if result.1.len() == 0 {
            player_1.push(*current_cards.0);
            player_1.push(*current_cards.1);
        } else {
            panic!("Bug?");
        }
    } else {
        play_combat_round(&mut player_1, &mut player_2);
    }
    (player_1, player_2, recursion_history)
}

fn play(mut player_1: Vec<u32>, mut player_2: Vec<u32>, part: u8) -> (Vec<u32>, Vec<u32>) {
    let mut recursion_history: HashMap<Vec<u32>, (bool, bool)> = HashMap::new();
    while player_1.len() != 0 && player_2.len() != 0 {
        match part {
            1 => play_combat_round(&mut player_1, &mut player_2),
            2 => {
                let check_history = (recursion_history.get(&player_1).unwrap_or(&(false, false)).0, recursion_history.get(&player_2).unwrap_or(&(false, false)).1);    
                if check_history.0 || check_history.1 {
                    player_2 = vec!();
                    break;
                } 
                let result = play_recursive_combat_round(player_1, player_2, recursion_history);
                player_1 = result.0;
                player_2 = result.1;
                recursion_history = result.2;
            },
            _ => panic!("Invalid part. [1, 2]"),
        }
    }
    (player_1, player_2)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide input file path.");
        std::process::exit(1);
    }
    
    let file_content = parse_file(&args[1]);
    let combat_game = play(file_content.0.clone(), file_content.1.clone(), 1);
    println!("Part 1 result: {}", get_score(&combat_game).unwrap());
    let game = play(file_content.0.clone(), file_content.1.clone(), 2);
    println!("{:?}", game);
    println!("Part 2 result: {}", get_score(&game).unwrap());
}
