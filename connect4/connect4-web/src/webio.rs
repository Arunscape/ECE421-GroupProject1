use super::canvas::Canvas;
use super::controller;
use super::{request_animation_frame, seconds};
use connect4_lib::{game::Board, game::BoardState, game::ChipDescrip, game::Game, GameIO};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

enum GameState {
    GetMove,
    WaitingForRemote,
    PlayingMove,
    GameOver,
}

pub struct WebIO {
    canvas: Canvas,
    game: Game,
    game_state: GameState,
}

impl WebIO {
    fn new(game: Game) -> Self {
        Self {
            game,
            canvas: Canvas::new("#canvas", 200, 200),
            game_state: GameState::GetMove,
        }
    }

    pub fn do_game_iteration(&self, delta: f64) {
        self.do_iteration_inputs(delta);
        self.do_iteration_updates(delta);
        self.do_iteration_renders(delta);
    }

    pub fn do_iteration_inputs(&self, delta: f64) {
        if self.canvas.is_mouse_pressed() {
            let loc = self.canvas.get_mouse_loc();
            let pos = controller::canvas_loc_to_column(&self.canvas, loc.0, loc.1, self.game.get_board());
            console_log!("{} -> {:?}", loc.0, pos);
        }
    }
    pub fn do_iteration_updates(&self, delta: f64) {}
    pub fn do_iteration_renders(&self, delta: f64) {
        controller::draw_gameboard(&self.canvas, &self.game.get_board());
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
