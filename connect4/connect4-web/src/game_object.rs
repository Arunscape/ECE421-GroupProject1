use crate::canvas::Canvas;
use crate::coms;
use crate::controller;
use crate::{request_animation_frame, set_timeout};
#[macro_use]
use crate::{console_log, log};
use connect4_lib::game::{Board, BoardState, Chip, ChipDescrip, Game, PlayerType};
use connect4_lib::ai::{AIConfig};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::jq::{mpsc, JReceiver, JSender};

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
    sender: JSender<Msg>,
    gameid: String,
}

#[derive(Debug)]
pub struct ChipAnimation {
    pub chip: ChipDescrip,
    pub x: isize,
    pub final_y: isize,
    pub y: f64,
    pub vy: f64,
    pub height: usize,
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
    FinishedAnimation,
    ServerReceived,
    AIThought((isize, ChipDescrip)),
}

impl GameObject {
    pub fn new(canvas: Canvas, game: Game, gameid: String) -> Self {
        let (sender, message_receiver) = mpsc();
        let mut slf = GameOnThread {
            canvas,
            game,
            game_state: GameState::WaitingForMove(PlayerType::Local),
            message_receiver,
            gameid,
            sender: sender.clone(),
        };
        slf.move_to_state(slf.derive_state_from_board());

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
        let mut time = crate::seconds();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let newtime = crate::seconds();
            let delta = newtime - time;
            time = newtime;
            thread_data.get_message(delta);
            let timeout = match thread_data.game_state {
                GameState::WaitingForMove(PlayerType::Remote) => 1000,
                GameState::WaitingForMove(PlayerType::Local) => 200,
                GameState::WaitingForMove(PlayerType::AI(_)) => 400,
                GameState::PlayingMove(_) => 20,
                GameState::GameOver(_) => 500,
            };
            set_timeout(f.borrow().as_ref().unwrap(), timeout);
        }) as Box<dyn FnMut()>));

        set_timeout(g.borrow().as_ref().unwrap(), 0);
    }
}

impl GameOnThread {
    pub fn play_move(&mut self, chip: Chip) {
        let chip_descrip = chip.get_descrip();
        let board = self.game.get_board();
        let loc = chip.get_x();

        self.game.play(loc, chip_descrip);
        self.game_state = GameState::PlayingMove(Box::from(self.derive_state_from_board()));
        start_animation(&self.canvas, &self.game.get_board(), self.sender.clone());
    }

    pub fn move_to_state(&mut self, next: GameState) {
        console_log!("Starting state: {:?}", next);
        self.game_state = next;
        match self.game_state {
            GameState::WaitingForMove(PlayerType::Remote) => self.request_game_from_server(),
            GameState::WaitingForMove(PlayerType::Local) => {},
            GameState::WaitingForMove(PlayerType::AI(config)) => self.request_move_from_ai(config),
            GameState::GameOver(board_state) => self.end_game(board_state),
            _ => {}
        }
    }

    pub fn get_message(&mut self, delta: f64) {
        let msg = self.message_receiver.recv();
        // console_log!("[{}] Got Message: {:?}", delta, msg);
        match msg {
            Some(Msg::FinishedAnimation) => {
                if let GameState::PlayingMove(next) = self.game_state.clone() {
                    self.repaint();
                    self.move_to_state(*next);
                }
            },
            Some(Msg::KeyPressed(key_code)) => {},
            Some(Msg::Clicked(loc)) => {
                let col = controller::canvas_loc_to_column(
                    &self.canvas,
                    loc.0,
                    loc.1,
                    self.game.get_board(),
                );
                if let Some(col) = col {
                    self.handle_click(col);
                }
            },
            Some(Msg::ServerReceived) => {
                console_log!("Got Game Data");
            },
            Some(Msg::AIThought((loc, ty))) => {
                self.play_move(Chip::new(loc, ty));
            },
            None => {}
        }
    }

    pub fn handle_click(&mut self, column_number: isize) {
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

    fn request_game_from_server(&self) {
        async fn getgamer(sender: JSender<Msg>, gameid: String) {
            let data = coms::getgame(&gameid).await;
            if let Some(gamedata) = data {
                console_log!("Server Data: {:?}", gamedata);
                sender.send(Msg::ServerReceived);
            }
        }
        spawn_local(getgamer(self.sender.clone(), self.gameid.clone()));
    }

    fn request_move_from_ai(&self, config: AIConfig) {
        async fn getgamer(sender: JSender<Msg>, mut game: Game, config: AIConfig) {
            let mov = connect4_lib::ai::get_best_move(&mut game, config);
            sender.send(Msg::AIThought(mov));
        }
        spawn_local(getgamer(self.sender.clone(), self.game.clone(), config));
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
        controller::draw_game_pieces(
            &self.canvas,
            &self.game.get_board(),
            &self.game.get_board().chips[..],
        );
    }
}

fn start_animation(canvas: &Canvas, board: &Board, sender: JSender<Msg>) {
    // create animation
    console_log!("Game chips: {:?}", board.chips);
    let chip = board.chips[board.chips.len() - 1];
    let (x, y) = board.last_move_loc();
    let mut ani = ChipAnimation {
        chip: chip.get_descrip(),
        x,
        final_y: y,
        y: 1100.0,
        vy: 0.0,
        height: board.height as usize,
    };

    // Actually start animation
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let canvas = Canvas::new(canvas.get_id());
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if controller::do_falling_piece_frame(&canvas, &mut ani) {
            request_animation_frame(f.borrow().as_ref().unwrap());
        } else {
            sender.send(Msg::FinishedAnimation);
        }
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
