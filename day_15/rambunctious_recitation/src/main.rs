use std::collections::HashMap;

struct Game {
    game_state: Vec<u32>,
    seen_numbers: HashMap<u32, usize>,
    last_number: u32,
}

impl Game {
    fn new(starting_state: Vec<u32>) -> Game {
        let mut game = Game {
            game_state: starting_state,
            seen_numbers: HashMap::new(),
            last_number: 0,
        };
        game.last_number = game.game_state.pop().unwrap();
        for (index, number) in game.game_state.iter().enumerate() {
            game.seen_numbers.insert(*number, index);
        }
        game
    }

    fn round_number(&self) -> usize {
        self.game_state.len() + 1
    }

    fn play_round(&mut self) {
        if !self.seen_numbers.contains_key(&self.last_number) {
            self.write_number_to_state();
            self.last_number = 0;
        } else {
            let matching_number_round = *self.seen_numbers.get(&self.last_number).unwrap() + 1;
            self.write_number_to_state();
            self.last_number = (self.game_state.len() - matching_number_round) as u32;
        }
    }

    fn play(&mut self, end_round: u32) {
        while (self.round_number() as u32) < end_round {
            self.play_round();
        }
    }

    fn write_number_to_state(&mut self) {
        self.game_state.push(self.last_number);
        self.seen_numbers
            .insert(self.last_number, self.game_state.len() - 1);
    }
}

fn main() {
    let mut game_1 = Game::new(vec!(7, 14, 0, 17, 11, 1, 2));
    game_1.play(2020);
    println!("Part 1 (Round 2020): {}", game_1.last_number);

    let mut game_2 = Game::new(vec!(7, 14, 0, 17, 11, 1, 2));
    game_2.play(30000000);
    println!("Part 2 (Round 30000000): {}", game_2.last_number);
}
