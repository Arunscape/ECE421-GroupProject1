pub mod game;
pub mod io;

use game::Game;
use io::{GameIO, TermIO};

pub fn play(game: &mut Game) {
    let mut is_over = false;
    while !is_over {
        TermIO::draw_board(game.get_board());
        let (loc, ty) = TermIO::get_move(&game);
        match game.play(loc, ty) {
            game::BoardState::Ongoing => {}
            game::BoardState::Invalid => {
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

mod web;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<web::App>();

    let c = web::canvas::Canvas::new("#canvas", 20, 20);
    c.draw(0, 0, &"red".into());
    Ok(())
}
