mod io;
mod game;

use io::{GameIO, TermDraw};
use game::{GameBoard, ConnectChip, ConnectColor, TotoChip, TotoType};


pub fn play() {
    println!("Hello, world!");

    let mut board = GameBoard::new(7, 6);
    ConnectChip::play(&mut board, 0, ConnectColor::Red);
    ConnectChip::play(&mut board, 3, ConnectColor::Yellow);
    ConnectChip::play(&mut board, 4, ConnectColor::Red);
    ConnectChip::play(&mut board, 4, ConnectColor::Yellow);
    TermDraw::draw_board(&board);

    println!();
    let mut board = GameBoard::new(7, 6);
    TotoChip::play(&mut board, 0, TotoType::O);
    TotoChip::play(&mut board, 3, TotoType::T);
    TotoChip::play(&mut board, 4, TotoType::T);
    TotoChip::play(&mut board, 4, TotoType::O);
    TermDraw::draw_board(&board);
}

