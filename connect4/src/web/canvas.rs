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

    pub fn draw(&self, x: u32, y: u32, colour: &JsValue) {
        assert!(x < self.width && y < self.height);

        self.context.set_fill_style(colour);

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
        self.context.set_fill_style(&"white".into());
        self.context.fill_rect(
            0.0,
            0.0,
            // f64::from(self.width * self.scaled_width),
            // f64::from(self.height * self.scaled_height),
            f64::from(self.canvas.width()),
            f64::from(self.canvas.height()),
        )
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

    pub fn draw_board(&self, board: &crate::game::Board) {
        self.draw_mask(board.width, board.height);
    }
}
