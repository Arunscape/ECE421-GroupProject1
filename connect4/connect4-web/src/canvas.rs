use connect4_lib::{
    game, game::Board, game::BoardState, game::ChipDescrip, game::Game, games, io, GameIO,
};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::cell::Cell;
use std::rc::Rc;

pub struct Canvas {
    pub canvas: web_sys::HtmlCanvasElement,
    pub context: web_sys::CanvasRenderingContext2d,
}

impl Canvas {
    pub fn new(canvas_id: String) -> Canvas {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(&canvas_id).unwrap();

        // setup HTML canvas and context
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

        // create actual Canvas object
        let mut my_can = Canvas { canvas, context };

        my_can
    }

    pub fn is_mouse_pressed(&self) -> bool {
        false
    }

    pub fn get_mouse_loc(&self) -> (i32, i32) {
        (0, 0)
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

    pub fn register_onclick_listener(&self, onclick: Box<FnMut(web_sys::MouseEvent)>) {
        let f = Closure::wrap(onclick);
        self.canvas.set_onclick(Some(f.as_ref().unchecked_ref()));
        f.forget();
    }
    pub fn register_keypress_listener(&self, onkey: Box<dyn FnMut(web_sys::KeyboardEvent)>) {
        let f = Closure::wrap(onkey);
        crate::window().set_onkeypress(Some(f.as_ref().unchecked_ref()));
        f.forget();
    }

    pub fn get_id(&self) -> String {
        self.canvas.id()
    }
}
