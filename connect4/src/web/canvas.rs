use crate::{game::Board, game::BoardState, game::ChipDescrip, game::Game, GameIO};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct Canvas {
    pub canvas: web_sys::HtmlCanvasElement,
    pub context: web_sys::CanvasRenderingContext2d,
    scaled_width: u32,
    scaled_height: u32,
    width: u32,
    height: u32,
}

impl Canvas {
    pub fn new(attr_id: &str, width: u32, height: u32) -> Canvas {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();

        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.begin_path();

        let scaled_width = canvas.width() / width;
        let scaled_height = canvas.height() / height;

        Canvas {
            canvas,
            context,
            scaled_width,
            scaled_height,
            width,
            height,
        }
    }

    pub fn draw_mask(&self, width: usize, height: usize) {
        self.context.save();
        self.context.set_fill_style(&"#00bfff".into());
        self.context.begin_path();
        for y in 0..height {
            for x in 0..width {
                self.context.arc(
                    75.0 * x as f64 + 100.0,
                    75.0 * y as f64 + 50.0,
                    25.0,
                    0.0,
                    2.0 * std::f64::consts::PI,
                );
                self.context
                    .rect(75.0 * x as f64 + 150.0, 75.0 * y as f64, -100.0, 100.0);
            }
        }
        self.context.fill();
        self.context.restore();
    }

    // TODO: rename this to draw_gameboard or something
    // also, change the use of thie function from lib.rs to
    // the new name
    pub fn draw_board(&self, board: &crate::game::Board) {
        self.draw_mask(board.width, board.height);
        // TODO: for each chip in board.chips,
        // call draw circle to draw the chip
    }

    pub fn draw_circle(&self, x: f64, y: f64, r: f64, fill: String, stroke: String) {
        self.context.save();
        self.context.set_fill_style(&fill.into());
        self.context.set_stroke_style(&stroke.into());
        self.context.begin_path();
        self.context.arc(x, y, r, 0.0, 2.0 * std::f64::consts::PI);
        self.context.fill();
        self.context.restore();
    }

    pub fn draw(&self) {
        // TODO: implement the finite state machine drawing
        /*
         match self.state {
             State::GetMove => {
                 call draw game board
                 call function to highlight column on which a player is moused over or, to show a ghost chip where it would go if they clicked, or something that gives feedback to show they can click to select a move.
             },
             State::DrawBoard => {
                 call draw game board
                 call function that draws an animation of a chip falling down, or appearing into the spot
             },
             State::GameOver => {
                 call function that draws a game over message and shows who wins
             }
         }
        */
        unimplemented!();
        /*
        let mut fg_color: &str;
        for y in 0..6 {
            for x in 0..7 {
                fg_color = "transparent";
                if (self.map[y][x] >= 1) {
                    fg_color = "#ff4136";
                } else if (this.map[y][x] <= -1) {
                    fg_color = "#ffff00";
                }
                self.draw_circle(
                    75.0 * x as f64 + 100.0,
                    75.0 * y as f64 + 50.0,
                    25.0,
                    fg_color.into(),
                    "black".into(),
                );
            }
        }
        */
    }

    pub fn clear(&self) {
        self.context.clear_rect(
            0.0,
            0.0,
            self.canvas.width().into(),
            self.canvas.height().into(),
        );
    }

    pub fn animate(column: usize, moove: usize, to_row: usize, cur_pos: usize, callback: fn()) {
        unimplemented!();
        /*
        let mut fg_color = "transparent";
        if moove >= 1 {
            fg_color = "#ff4136";
        } else if moove <= -1 {
            fg_color = "#ffff00";
        }
        if to_row * 75 >= cur_pos {
            self.clear();
            self.draw();
            self.draw_circle(75 * column + 100, cur_pos + 50, 25, fg_color, "black");
            self.draw_mask();
            web_sys::window().request_animation_frame(|| that.animate(column, moove, to_row, cur_pos + 25, callback)
        } else {
            callback();
        }
        */
    }
}

// TODO: so arun, this is going to be the tricky bit, none of these
// functions are actually going to do anything other then update the
// state of Canvas to the appropriate state. The hard part, is making
// sure none of these functions return until they are finished. That
// means, not having draw board return until the piece animation is
// done. Not having get move return until a move is selected, and not
// having display_gameover return til the person is node viewing the
// screen
impl GameIO for Canvas {
    fn draw_board(&self, game: &Board) {
        self.draw()
    }

    fn get_move(&self, game: &Game) -> (usize, ChipDescrip) {
        unimplemented!();
    }

    fn display_gameover(&self, ending: BoardState) {
        unimplemented!();
    }
}
