use crate::canvas::Canvas;
use crate::controller;
#[macro_use]
use crate::{console_log, log};
use connect4_lib::game::{BoardState, Chip, ChipDescrip, Game, PlayerType};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use std::sync::mpsc;

pub struct GameObject {
    canvas: Canvas,
    game: Game,
    game_state: GameState,
    falling_loc: Option<(isize, f64, f64)>, // x, y, vy
    message_receiver: mpsc::Receiver<Msg>,
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
        let (sender, message_receiver) = mpsc::channel();
        let mut slf = Self {
            canvas,
            game,
            falling_loc: None,
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

        slf
    }

    pub fn play_move(&mut self, chip: Chip) {
        let chip_descrip = chip.get_descrip();
        let board = self.game.get_board();
        let loc = chip.get_x();

        // controller::animate_falling_piece(self.canvas, chip: connect4_lib::game::ChipDescrip, board: &Board, loc: (isize, f64, f64))
        self.game_state = GameState::WaitingForMove(self.game.current_player().player_type);
    }

    pub fn wait_for_messages(&mut self) {
        match self.message_receiver.recv() {
            Ok(Msg::KeyPressed(key_code)) => {},
            Ok(Msg::Clicked(loc)) => {
                let col = controller::canvas_loc_to_column(&self.canvas, loc.0, loc.1, self.game.get_board());
                if let Some(col) = col {
                    self.handle_click(col);
                }
            },
            Err(err) => {},
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
    }

}
