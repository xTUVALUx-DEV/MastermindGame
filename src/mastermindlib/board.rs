use rand::Rng;
use rgb::RGB8;
use std::cmp;
use std::collections::HashMap;

const COLORS: [RGB8; 9] = [
    RGB8 {
        r: 241,
        g: 196,
        b: 15,
    },
    RGB8 {
        r: 26,
        g: 188,
        b: 156,
    },
    RGB8 {
        r: 52,
        g: 152,
        b: 219,
    },
    RGB8 {
        r: 155,
        g: 89,
        b: 182,
    },
    RGB8 {
        r: 192,
        g: 57,
        b: 43,
    },
    RGB8 {
        r: 243,
        g: 156,
        b: 18,
    },
    RGB8 {
        r: 22,
        g: 160,
        b: 133,
    },
    RGB8 {
        r: 41,
        g: 128,
        b: 185,
    },
    RGB8 {
        r: 142,
        g: 68,
        b: 173,
    },
];

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
// Todo: Implement Default
pub struct BoardSettings {
    pub colors: Vec<RGB8>,
    pub code_length: u8,
    pub max_tries: u8,
    pub is_ended: bool,
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

    pub fn generate_colors(&mut self, number: i16) {
        self.colors = COLORS[..number as usize].to_vec();
    }

    pub fn with_n_colors(mut self, number: i16) -> Self {
        self.colors = COLORS[..number as usize].to_vec();
        self
    }
}

#[derive(Debug)]
pub struct MastermindBoard {
    pub settings: BoardSettings,
    pub state: BoardState,
}

impl MastermindBoard {
    pub fn new(settings: BoardSettings) -> Self {
        Self {
            state: BoardState::new(&settings),
            settings,
        }
    }

    pub fn guess(&mut self, guess: &Guess) -> GameState {
        if self.state.guesses.len() >= self.settings.max_tries as usize {
            return GameState::GameEnd { 0: false };
        }

        let response = self.state.guess(guess);

        // Check if that was the last try
        if let GameState::GuessAnswer(..) = response {
            if self.state.guesses.len() >= self.settings.max_tries as usize {
                return GameState::GameEnd { 0: false };
            }
        }
        response
    }
}

impl Default for MastermindBoard {
    fn default() -> Self {
        Self::new(BoardSettings::default())
    }
}

#[derive(Debug)]
pub struct BoardState {
    pub code: Vec<u8>,
    pub guesses: Vec<Guess>,
    pub answers: Vec<GameState>,
}

impl BoardState {
    pub fn new(settings: &BoardSettings) -> Self {
        let mut rng = rand::rng();

        // Generate a random solution
        let solution: Vec<u8> = (0..settings.code_length)
            .map(|_| rng.random_range(0..settings.colors.len() as u8))
            .collect();

        Self {
            guesses: Vec::new(),
            answers: Vec::new(),
            code: solution,
        }
    }

    fn guess(&mut self, guess: &Guess) -> GameState {
        self.guesses.push(guess.clone());
        // Convert the code numbers to a hashmap
        let numbers_in_code: HashMap<_, _> = self
            .code
            .iter()
            .map(|&n| (n, self.code.iter().filter(|&&x| x == n).count()))
            .collect();

        // Compare the number count for each number
        let right_numbers_count: u8 = numbers_in_code
            .iter()
            .map(|(&k, &v)| cmp::min(v, guess.0.iter().filter(|&&x| x == k).count()) as u8)
            .sum();

        // Check each index
        let right_position_count: u8 = guess
            .0
            .iter()
            .zip(self.code.iter())
            .filter(|(x, y)| x == y)
            .count() as u8;

        let answer = GameState::GuessAnswer {
            0: right_position_count,
            1: right_numbers_count - right_position_count,
        };
        self.answers.push(answer.clone());

        if usize::from(right_position_count) == self.code.len() {
            return GameState::GameEnd { 0: true };
        }

        answer
    }
}

#[derive(Debug, Clone)]
pub struct Guess(pub Vec<u8>);

#[derive(Debug, Clone)]
pub enum GameState {
    GuessAnswer(u8, u8),
    // (#right_pos, #right_not_pos)
    GameEnd(bool),
    // (has_won)
}

impl GameState {}
