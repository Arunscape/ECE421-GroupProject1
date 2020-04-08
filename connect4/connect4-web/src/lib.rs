use connect4_lib::game::Game;
use connect4_lib::games;
use connect4_lib::io::{GameIO, TermIO};

mod web;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<web::App>();

    let game = games::connect4();
    let c = web::canvas::Canvas::new("#canvas", 20, 20);

    let mut game = games::connect4_ai();
    connect4_lib::play(&mut game, c);

    Ok(())
}
