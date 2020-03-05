mod io;
pub mod game;

use io::{GameIO, TermIO};
use game::{Game};


pub fn play(game: &mut Game) {
    let mut is_over = false;
    while !is_over {
        TermIO::draw_board(game.get_board());
        let (loc, ty) = TermIO::get_move(&game);
        match game.play(loc, ty) {
            game::BoardState::Ongoing => {},
            x => { TermIO::display_gameover(x); is_over = true; },
        }
    }
    TermIO::draw_board(game.get_board());

    // for debugging
    game.print_moves();
    println!();
}

