mod io;
mod game;

use io::{GameIO, TermIO};
use game::{GameBoard, Game, GameType::*};


pub fn play() {
    println!("Hello, world!");

    let board = GameBoard::new(7, 6);
    let mut game = Game::new(board, Toto);
    loop {
        TermIO::draw_board(game.get_board());
        let (loc, ty) = TermIO::get_move(&game);
        game.play(loc, ty);
    }
}

