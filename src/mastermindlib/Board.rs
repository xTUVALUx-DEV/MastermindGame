use rgb::RGB8;
use std::cmp;
use std::collections::HashMap;
use rand::prelude::IndexedRandom;

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
// Todo: Implement Default
pub struct BoardSettings {
    pub colors: Vec<RGB8>,
    pub code_length: u8,
    pub max_tries: u8
}

impl BoardSettings {
    pub fn colors(mut self, colors: Vec<RGB8>) -> BoardSettings {
        self.colors = colors;
        self
    }
    pub fn code_length(mut self, code_length: u8) -> BoardSettings {
        self.code_length = code_length;
        self
    }

    pub fn max_tries(mut self, tries: u8) -> BoardSettings {
        self.max_tries = tries;
        self
    }
}


#[derive(Debug)]
pub struct MastermindBoard {
    pub settings: BoardSettings,
    pub state: BoardState
}

impl MastermindBoard {
    pub fn new(settings: BoardSettings) -> Self {
        Self { state: BoardState::new(&settings), settings }
    }

    pub fn guess(&mut self, guess: &Guess) -> GameState {
        self.state.guess(guess)
    }
}

impl Default for MastermindBoard {
    fn default() -> Self {
        Self::new(BoardSettings::default())
    }
}


#[derive(Debug)]
struct BoardState {
    code: Vec<u8>,
    guesses: Vec<Guess>,
    answers: Vec<GameState>
}

impl BoardState {
    pub fn new(settings: &BoardSettings) -> Self {
        let mut rng = rand::rng();

        // Generate a random solution
        let solution: Vec<u8> = (0..settings.code_length).collect::<Vec<_>>()
            .choose_multiple(&mut rng, usize::from(settings.code_length))
            .copied().collect();

        Self { guesses: Vec::new(), answers: Vec::new(), code: solution }
    }

    fn guess(&mut self, guess: &Guess) -> GameState {
        self.guesses.push(guess.clone());
        // Convert the code numbers to a hashmap
        let numbers_in_code: HashMap<_, _> = self.code.iter() 
            .map(|&n| (n, self.code.iter().filter(|&&x| x == n).count()))
            .collect();

        // Compare the number count for each number
        let right_numbers_count: u8 = numbers_in_code.iter()
            .map(|(&k, &v)|
                cmp::min(v, guess.0.iter().filter(|&&x| x == k).count()) as u8)
            .sum();
        
        // Check each index
        let right_position_count: u8 = guess.0.iter()
            .zip(self.code.iter()).filter(|(x, y)| x == y).count() as u8;

        if usize::from(right_numbers_count) == self.code.len() {
            return GameState::GameEnd { };
        }

        let answer = GameState::GuessAnswer { 0: right_position_count, 1: right_numbers_count-right_position_count };
        self.answers.push(answer.clone());
        answer
    }
}

#[derive(Debug, Clone)]
pub struct Guess(pub Vec<u8>);


#[derive(Debug, Clone)]
pub enum GameState {
    GuessAnswer(u8, u8),
    GameEnd,
}

impl GameState {

}
// (#right_pos, #right_not_pos)

