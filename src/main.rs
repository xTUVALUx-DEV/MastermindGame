pub mod mastermindlib;
use mastermindlib::board::MastermindBoard;
use mastermindlib::board::BoardSettings;
use mastermindlib::board::Guess;

use rgb::RGB8;

fn main() {
    let settings = BoardSettings::default()
        .colors(vec![RGB8::new(255, 0, 0), RGB8::new(255, 0, 255), RGB8::new(255, 255, 0)])
        .code_length(4);
    let mut board = MastermindBoard::new(settings);
    board.guess(&Guess { 0: vec![1,2,2,2]});
    println!("{:?}", board);
}

