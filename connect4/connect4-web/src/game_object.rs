use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d}
use crate::canvas::Canvas;
use connect4_lib::game::{Game, BoardState, PlayerType, Chip, ChipDescrip};

pub struct GameObject {
    canvas: Canvas,
    game: Game,
    game_state: GameState,
    falling_loc: Option<(isize, f64, f64)>, // x, y, vy
}

#[derive(Clone, Debug)]
enum GameState {
    WaitingForMove(PlayerType),
    PlayingMove(Box<GameState>),
    GameOver(BoardState),
}

impl GameObject {
    pub fn new(canvas: Canvas, game: Game) -> Self {
        let game_state = self.derive_state_from_board();

        let bounds = canvas.canvas.get_bounding_client_rect();
        let left = bounds.x() as i32;
        let top = bounds.y() as i32;
        let onclick = Box::new(|e: web_sys::MouseEvent| {
            let loc = ((e.client_x() - left, e.client_y() - top));
            let col = controller::canvas_loc_to_column(&self.canvas, loc.0, loc.1, self.game.get_board());
            self.handle_click(col);
        });

        let onkeypress = Box::new(|e: web_sys::KeyEvent| {
            todo!();
        });

        canvas.register_onclick_listener(onclick);
        canvas.register_keypress_listener(onkeypress);

        Self { canvas, game. falling_loc: None,  game_state}
    }

    pub fn play_move(&mut self, chip: Chip){
        let chip_descrip = chip.get_descrip();
        let board = self.game.get_board();
        let loc = chip.get_x();

       // controller::animate_falling_piece(self.canvas, chip: connect4_lib::game::ChipDescrip, board: &Board, loc: (isize, f64, f64))
       self.game_state = WaitingForMove(board.current_player);
    }

    pub fn handle_click(&mut self, column_number: isize)  {

        let state = self.derive_state_from_board();

        match state {
            GameState::WaitingForMove(player_type) => controller::message(self.canvas, "Wait for your opponent to make a move!"),
            GameState::GameOver(board_state) => self.end_game(board_state),
            GameState::PlayingMove(boxed_game_state) => {
                let chip_descrip = self.game.current_player().chip_options[0];
                let chip = Chip::new(column_number, chip_descrip);
                self.play_move(chip);
            },
        }

    }

    pub fn end_game(&self, board_state: BoardState){
                controller::draw_gameboard(&self.canvas, &self.game.get_board());
                controller::draw_game_pieces(
                    &self.canvas,
                    &self.game.get_board(),
                    &self.game.get_board().chips[..],
                );
        let message = match board_state{
            BoardState::Win(player)=> format!("Game Over: Player {} Wins!", player),
            BoardState::Draw => "Game Over. Draw.. :(",

        };
        controller::message(&self.canvas, message);
    }

    pub fn handle_keyboard_event(&mut self, key: char){
        todo!();
    }

    fn derive_state_from_board(&self) -> GameState {
        match self.game.compute_board_state(){
            BoardState::Draw => GameState::GameOver(BoardState::Draw),
            BoardState::Win(winning_player_index)=> GameState::GameOver(BoardState::Win(winning_player_index)),
            BoardState::Invalid => panic!("Board state must not be invalid"),
            BoardState::Ongoing =>  GameState::WaitingForMove(self.game.current_player().player_type),
        }
    }

    fn wait_for_move(&self){
        // poll the server?
        //self.game = coms::getgame(game_id).await.unwrap().game;
        todo!();
    }



    // get user input from canvas
    // call functions in controller
    // replaces webio
    //another thing the game object needs to do is get the desired move from the player in games like toot and Otto. Right now, I hard coded in, get the first move from the move list
}
