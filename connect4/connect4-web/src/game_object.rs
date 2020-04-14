use crate::canvas::Canvas;
use crate::controller;
use crate::set_timeout;
#[macro_use]
use crate::{console_log, log};
use connect4_lib::game::{BoardState, Chip, ChipDescrip, Game, PlayerType};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::jq::{JReceiver, JSender, mpsc};

use std::cell::RefCell;
use std::rc::Rc;


pub struct GameObject {
    channel: JSender<Msg>,
}

struct GameOnThread {
    canvas: Canvas,
    game: Game,
    game_state: GameState,
    message_receiver: JReceiver<Msg>,
}

struct ChipAnimation {
    x: isize,
    final_y: isize,
    y: f64,
    vy: f64,
}

#[derive(Clone, Debug)]
enum GameState {
    WaitingForMove(PlayerType),
    PlayingMove(Box<GameState>),
    GameOver(BoardState),
}

#[derive(Clone, Debug)]
enum Msg {
    Clicked((i32, i32)),
    KeyPressed(u32),
}


impl GameObject {
    pub fn new(canvas: Canvas, game: Game) -> Self {
        let (sender, message_receiver) = mpsc();
        let mut slf = GameOnThread {
            canvas,
            game,
            game_state: GameState::WaitingForMove(PlayerType::Local),
            message_receiver,
        };
        let game_state = slf.derive_state_from_board();
        slf.game_state = game_state;

        let mouse_sender = sender.clone();
        let bounds = slf.canvas.canvas.get_bounding_client_rect();
        let left = bounds.x() as i32;
        let top = bounds.y() as i32;
        let onclick: Box<FnMut(web_sys::MouseEvent)> = Box::new(move |e: web_sys::MouseEvent| {
            let loc = ((e.client_x() - left, e.client_y() - top));
            mouse_sender.send(Msg::Clicked(loc));
        });

        let key_sender = sender.clone();
        let onkeypress = Box::new(move |e: web_sys::KeyboardEvent| {
            key_sender.send(Msg::KeyPressed(e.key_code()));
        });

        slf.canvas.register_onclick_listener(onclick);
        slf.canvas.register_keypress_listener(onkeypress);

        slf.repaint();

        let handle = GameObject { channel: sender };
        handle.start_listener_thread(slf);
        handle
    }

    fn start_listener_thread(&self, mut thread_data: GameOnThread) {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            thread_data.get_message();
            set_timeout(f.borrow().as_ref().unwrap(), 200);
        }) as Box<dyn FnMut()>));

        set_timeout(g.borrow().as_ref().unwrap(), 0);
    }
}

impl GameOnThread {
    pub fn play_move(&mut self, chip: Chip) {
        let chip_descrip = chip.get_descrip();
        let board = self.game.get_board();
        let loc = chip.get_x();

        // controller::animate_falling_piece(self.canvas, chip: connect4_lib::game::ChipDescrip, board: &Board, loc: (isize, f64, f64))
        self.game_state = GameState::WaitingForMove(self.game.current_player().player_type);
    }

    pub fn get_message(&mut self) {
        let msg = self.message_receiver.recv();
        console_log!("Got Message: {:?}", msg);
        match msg {
            Some(Msg::KeyPressed(key_code)) => {},
            Some(Msg::Clicked(loc)) => {
                let col = controller::canvas_loc_to_column(&self.canvas, loc.0, loc.1, self.game.get_board());
                if let Some(col) = col {
                    self.handle_click(col);
                }
            },
            None => {},
        }
    }

    pub fn handle_click(&mut self, column_number: isize) {
        console_log!("clicked column {}", column_number);
        let state = self.derive_state_from_board();

        match state {
            GameState::GameOver(board_state) => self.end_game(board_state),
            GameState::PlayingMove(boxed_game_state) => { /* Ignore clicks while animating */ }
            GameState::WaitingForMove(player_type) => match player_type {
                PlayerType::Local => {
                    let chip_descrip = self.game.current_player().chip_options[0];
                    let chip = Chip::new(column_number, chip_descrip);
                    self.play_move(chip);
                }
                _ => controller::message(
                    &self.canvas,
                    String::from("Wait for your opponent to make a move!"),
                ),
            },
        };
    }

    pub fn end_game(&self, board_state: BoardState) {
        controller::draw_gameboard(&self.canvas, &self.game.get_board());
        controller::draw_game_pieces(
            &self.canvas,
            &self.game.get_board(),
            &self.game.get_board().chips[..],
        );
        let message = match board_state {
            BoardState::Win(player) => format!("Game Over: Player {} Wins!", player),
            BoardState::Draw => format!("Game Over. Draw.. :("),
            _ => format!("Game not over?"),
        };
        controller::message(&self.canvas, message);
    }

    pub fn handle_keyboard_event(&mut self, key: char) {
        todo!();
    }

    fn handle_server_event(&self) {
        todo!();
    }

    fn derive_state_from_board(&self) -> GameState {
        match self.game.compute_board_state() {
            BoardState::Draw => GameState::GameOver(BoardState::Draw),
            BoardState::Win(winning_player_index) => {
                GameState::GameOver(BoardState::Win(winning_player_index))
            }
            BoardState::Invalid => panic!("Board state must not be invalid"),
            BoardState::Ongoing => {
                GameState::WaitingForMove(self.game.current_player().player_type)
            }
        }
    }

    fn repaint(&self) {
        controller::draw_gameboard(&self.canvas, &self.game.get_board());
        controller::draw_game_pieces(&self.canvas, &self.game.get_board(), &self.game.get_board().chips[..]);
    }

}

fn start_animation() {
}

