use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d}
use crate::canvas::Canvas;
use connect4_lib::game::Game;

pub struct GameObject {
    canvas: Canvas,
    game: Game
}

impl GameObject {
    pub fn new(canvas: Canvas, game: Game) -> Self {
        Self { canvas, game }
    }

    pub fn play_move(&mut self, chip: Chip){

    }

    pub fn user_input(&self) -> isize {

    }

    pub fn handle_click(&mut self, column_number: isize)  {
    //    let Chip 
    }

    

    // get user input from canvas
    // call functions in controller
    // replaces webio
}
