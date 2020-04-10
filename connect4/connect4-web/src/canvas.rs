use connect4_lib::{
    game, game::Board, game::BoardState, game::ChipDescrip, game::Game, games, io, GameIO,
};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;


pub struct Canvas {
    pub canvas: web_sys::HtmlCanvasElement,
    pub context: web_sys::CanvasRenderingContext2d,
    width: u32,
    height: u32,
    press_count: usize,
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

        Canvas {
            canvas,
            context,
            width,
            height,
            press_count: 0,
        }
    }

    pub fn isMousePressed() -> bool {
        press_count > 0
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
        // TODO: draw chip going into place
    }
}

