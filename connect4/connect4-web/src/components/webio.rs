use crate::canvas::Canvas;
use crate::log;
use crate::controller;
use crate::window;
use crate::{request_animation_frame, seconds};
use connect4_lib::{game::Board, game::BoardState, game::ChipDescrip, game::Game, GameIO};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

#[derive(Clone)]
enum GameState {
    GetMove,
    WaitingForRemote,
    WaitingForLocal,
    PlayingMove(Box<GameState>),
    GameOver(BoardState),
}

pub struct WebIO {
    canvas: Canvas,
    game: Game,
    game_state: GameState,
    running: bool,
    over_column: Option<usize>,
    falling_loc: Option<(usize, f64, f64)>, // x, y, vy
}

impl WebIO {
    fn new(game: Game) -> Self {
        Self {
            game,
            canvas: Canvas::new("#canvas", 200, 200),
            game_state: GameState::GetMove,
            running: true,
            over_column: None,
            falling_loc: None,
        }
    }

    fn do_game_iteration(&mut self, delta: f64) {
        self.do_iteration_inputs(delta);
        self.do_iteration_updates(delta);
        self.do_iteration_renders(delta);
    }

    fn do_iteration_inputs(&mut self, delta: f64) {
        let loc = self.canvas.get_mouse_loc();
        self.over_column =
            controller::canvas_loc_to_column(&self.canvas, loc.0, loc.1, self.game.get_board());
    }
    fn do_iteration_updates(&mut self, delta: f64) {
        if self.canvas.is_mouse_pressed() && self.over_column.is_some() {
            if let GameState::GetMove = self.game_state {
                self.play_local_move();
            }
        }

        if let GameState::GameOver(_) = self.game_state.clone() {
            if self.canvas.is_mouse_pressed() {
                self.finish();
            }
        }

        if let GameState::PlayingMove(next) = self.game_state.clone() {
            if let Some(falling) = self.falling_loc {
                self.falling_loc = controller::update_falling_piece(self.game.get_board(), falling, delta);
                if let None = self.falling_loc {
                    self.game_state = *next;
                }
            }
        }
    }

    fn do_iteration_renders(&self, delta: f64) {
        self.canvas.clear();
        match self.game_state {
            GameState::WaitingForRemote => {
                controller::draw_gameboard(&self.canvas, &self.game.get_board());
                controller::draw_game_pieces(
                    &self.canvas,
                    &self.game.get_board(),
                    &self.game.get_board().chips[..],
                );
                // TODO: display something to indicate
            }
            GameState::WaitingForLocal => {
                controller::draw_gameboard(&self.canvas, &self.game.get_board());
                controller::draw_game_pieces(
                    &self.canvas,
                    &self.game.get_board(),
                    &self.game.get_board().chips[..],
                );
                // TODO: display something to indicate
            }
            GameState::GetMove => {
                controller::draw_gameboard(&self.canvas, &self.game.get_board());
                controller::draw_game_pieces(
                    &self.canvas,
                    &self.game.get_board(),
                    &self.game.get_board().chips[..],
                );
                if let Some(col) = self.over_column {
                    controller::highlight_column(&self.canvas, col);
                }
            }
            GameState::PlayingMove(_) => {
                if let Some(fall) = self.falling_loc {
                    controller::animate_falling_piece(
                        &self.canvas,
                        self.game.get_board().chips[(self.game.get_board().chips.len() - 1)]
                            .get_descrip(),
                        &self.game.get_board(),
                        fall,
                    );
                }
                controller::draw_gameboard(&self.canvas, &self.game.get_board());
                controller::draw_game_pieces(
                    &self.canvas,
                    &self.game.get_board(),
                    &self.game.get_board().chips[0..(self.game.get_board().chips.len() - 1)],
                );
            }
            GameState::GameOver(BoardState::Draw) => {
                controller::message(&self.canvas, format!("Game Over: Draw :("));
            }
            GameState::GameOver(BoardState::Win(player)) => {
                controller::message(&self.canvas, format!("Game Over: Player {} Wins!", player));
            }
            GameState::GameOver(_) => {
                controller::message(&self.canvas, format!("Game Over: Error"));
            }
        }
    }

    fn play_local_move(&mut self) {
        let col = self
            .over_column
            .expect("play_local_move requires over_column");
        let move_type = self.game.current_player().chip_options[0];
        let res = self.game.play(col, move_type);

        self.game_state = match res {
            BoardState::Ongoing => GameState::PlayingMove(Box::from(GameState::GetMove)),
            BoardState::Invalid => self.game_state.clone(),
            BoardState::Draw => GameState::GameOver(res),
            BoardState::Win(_) => GameState::GameOver(res),
        };
        self.falling_loc = Some((col, 1100.0, 0.0)); // TODO: no magic numbers
    }

    fn finish(&mut self) {
        self.running = false;
    }

    pub fn play_with_game_loop(game: Game) {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let mut webio = WebIO::new(game);

        let mut time = seconds();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let curTime = seconds();
            let delta = curTime - time;
            time = curTime;
            webio.do_game_iteration(delta);

            if webio.running {
                request_animation_frame(f.borrow().as_ref().unwrap());
            }
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}

pub struct WebIOComponent {
    link: ComponentLink<Self>,
}
pub enum WebIOMsg {
    Back,
}
impl Component for WebIOComponent {
    type Message = WebIOMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }
    fn mounted(&mut self) -> ShouldRender {
        let game = connect4_lib::games::connect4();
        WebIO::play_with_game_loop(game);
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        window().history().unwrap().back();
        true
    }

    fn view(&self) -> Html {
        let back_callback = self.link.callback(|_| WebIOMsg::Back);
        html!{
            <div>
                <div>
                  <button onclick=back_callback> { "Back" } </button>
                </div>
                <canvas id="canvas" height="1080" width="1960" style="outline: black 3px solid; height: 500px; width: 900px;"/>
            </div>
        }
    }
}
