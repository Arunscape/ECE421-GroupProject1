use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d}
use crate::canvas::Canvas;
use connect4_lib::game::{Game, BoardState, PlayerType};

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

        Self { canvas, game. falling_loc: None,  game_state}
    }

    pub fn play_move(&mut self, chip: Chip){
        
    }

    pub fn user_input(&self) -> isize {

    }

    pub fn handle_click(&mut self, column_number: isize)  {
    //    let Chip 
    }

    pub fn handle_keyboard_event(&mut self, key: char){
        
    }

    fn derive_state_from_board(&self) -> GameState {
        match self.game.compute_board_state(){
            BoardState::Draw => GameState::GameOver(BoardState::Draw),
            BoardState::Win(winning_player_index)=> GameState::GameOver(BoardState::Win(winning_player_index)),
            BoardState::Invalid => panic!("Board state must not be invalid"),
            BoardState::Ongoing =>  GameState::WaitingForMove(self.game.current_player().player_type),
        }
    }

    

    // get user input from canvas
    // call functions in controller
    // replaces webio
    //another thing the game object needs to do is get the desired move from the player in games like toot and Otto. Right now, I hard coded in, get the first move from the move list
}
