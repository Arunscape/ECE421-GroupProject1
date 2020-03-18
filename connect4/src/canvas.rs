use crate::GameIO;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};

pub struct Canvas {
    pub canvas: CanvasElement,
    pub context: CanvasRenderingContext2d,
    scaled_width: u32,
    scaled_height: u32,
    width: u32,
    height: u32,
}

impl Canvas {
    pub fn new(attr_id: &str, width: u32, height: u32) -> Canvas {
        let canvas: CanvasElement = document()
            .query_selector(attr_id)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
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

    pub fn draw(&self, x: u32, y: u32, colour: &str) {
        assert!(x < self.width && y < self.height);

        self.context.set_fill_style_color(colour);

        let x = x * self.scaled_width;
        let y = y * self.scaled_height;

        self.context.fill_rect(
            f64::from(x),
            f64::from(y),
            f64::from(self.scaled_width),
            f64::from(self.scaled_height),
        );
    }

    pub fn clear_all(&self) {
        self.context.set_fill_style_color("white");
        self.context.fill_rect(
            0.0,
            0.0,
            // f64::from(self.width * self.scaled_width),
            // f64::from(self.height * self.scaled_height),
            f64::from(self.canvas.width()),
            f64::from(self.canvas.height()),
        )
    }
}

impl GameIO for Canvas {
    fn draw_board(&self, game: &Board) {
        self.draw()
    }

    fn get_move(&self, game: &Game) {
        unimplemented!();
    }

    fn display_gameover(ending: BoardState) {
        unimplemented!();
    }
}
