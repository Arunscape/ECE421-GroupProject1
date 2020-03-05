mod io;
mod game;

use io::{GameIO, TermIO};
use game::{Board, Game, GameType::*};


pub fn play() {
    println!("Hello, world!");

    let board = Board::new(7, 6);
    let mut game = Game::new(board, Connect4);
    let mut is_over = false;
    while !is_over {
        TermIO::draw_board(game.get_board());
        let (loc, ty) = TermIO::get_move(&game);
        match game.play(loc, ty) {
            game::BoardState::Ongoing => {},
            x => { TermIO::display_gameover(x); is_over = true; },
        }
    }
    game.print_moves();
}

