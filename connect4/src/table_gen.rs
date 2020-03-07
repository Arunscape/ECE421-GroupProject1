use connect_game::ai::searcher::*;

use connect_game::game::GameType::*;
use connect_game::game::ChipDescrip::*;
use connect_game::game::connect4::ConnectColor::*;
use connect_game::game::BoardState;
use connect_game::game::Game;
use connect_game::io::{GameIO, TermIO};

fn main() {
    // gen_table(7, 6)
    let board = connect_game::game::Board::new(7, 6);
    let mut game = connect_game::game::Game::new(board, Connect4);
    play_ai(&mut game);
}

pub fn play_ai(game: &mut Game) {
    let mut is_over = false;
    while !is_over {
        TermIO::draw_board(game.get_board());
        let (loc, ty) = if game.get_turn() % 2 == 0 {
            TermIO::get_move(&game)
        } else {
            (get_best_move(game), Connect(Yellow))
        };
        match game.play(loc, ty) {
            BoardState::Ongoing => {}
            BoardState::Invalid => {
                println!("\n\nInvalid move.");
                game.undo_move();
            }
            x => {
                TermIO::display_gameover(x);
                is_over = true;
            }
        }
    }
    TermIO::draw_board(game.get_board());

    // for debugging
    game.print_moves();
    println!();
}
