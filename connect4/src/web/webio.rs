use crate::{game::Board, game::BoardState, game::ChipDescrip, game::Game, GameIO};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use super::canvas::Canvas;
use super::controller;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}


pub struct WebIO {
    canvas: Canvas
}

impl WebIO {
    pub fn new() -> Self {
        Self {
            canvas: Canvas::new("#canvas", 200, 200)
        }
    }
}

impl GameIO for WebIO {
    fn draw_board(&self, board: &Board) {
        controller::draw_gameboard(&self.canvas, board);
    }

    fn get_move(&self, game: &Game) -> (usize, ChipDescrip) {
        (1, game.current_player().chip_options[0])
    }

    fn display_gameover(&self, ending: BoardState) {
        alert("Game over");
    }
}
