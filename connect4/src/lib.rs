pub mod game;
pub mod games;
pub mod io;
pub mod ai;

use game::Game;
use io::{GameIO, TermIO};

pub fn play(game: &mut Game, io: impl GameIO) {
    let mut is_over = false;
    while !is_over {
        io.draw_board(game.get_board());
        let (loc, ty) = match game.current_player().player_type {
            game::PlayerType::Local => io.get_move(game),
            game::PlayerType::AI(ai) => ai::get_best_move(game, ai),
        };
        match game.play(loc, ty) {
            game::BoardState::Ongoing => {}
            game::BoardState::Invalid => {
                println!("\n\nInvalid move.");
            }
            x => {
                io.display_gameover(x);
                is_over = true;
            }
        }
    }
    io.draw_board(game.get_board());

    // for debugging
    game.print_moves();
    println!();
}

mod web;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<web::App>();

    let game = crate::games::connect4();
    let c = web::canvas::Canvas::new("#canvas", 20, 20);

    c.draw_board(game.get_board());

    Ok(())
}
