use crate::{game::Board, game::BoardState, game::ChipDescrip, game::Game, GameIO};
use super::canvas::Canvas;
use super::controller;
use super::{request_animation_frame, seconds};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use std::time::{Duration, Instant};
use std::cell::RefCell;
use std::rc::Rc;


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


pub struct WebIO {
    canvas: Canvas,
    game: Game
}

impl WebIO {
    fn new(game: Game) -> Self {
        Self {
            game,
            canvas: Canvas::new("#canvas", 200, 200),
        }
    }

    pub fn do_game_iteration(&self, delta: f64) {
        console_log!("delta {}", delta);
    }

    pub fn play_with_game_loop(game: Game) {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let webio = WebIO::new(game);

        let mut time = seconds();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let curTime = seconds();
            let delta = curTime - time;
            time = curTime;

            webio.do_game_iteration(delta);

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}

