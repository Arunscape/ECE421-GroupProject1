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
    width: u32,
    height: u32,
    press_count: Rc<Cell<isize>>,
    mouse_loc: Rc<Cell<(i32, i32)>>,
}

impl Canvas {
    pub fn new(attr_id: &str, width: u32, height: u32) -> Canvas {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();

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

        // Setup canvas mouse stuff
        let presser = Rc::new(Cell::new(0));
        let presser1 = presser.clone();
        let presser2 = presser.clone();
        let loc = Rc::new(Cell::new((0, 0)));
        let loc2 = loc.clone();

        let bounds = canvas.get_bounding_client_rect();
        let left = bounds.x() as i32;
        let top = bounds.y() as i32;

        let down = Closure::wrap(Box::new(move || {
            presser1.set(presser1.get() + 1);
        }) as Box<dyn FnMut()>);
        let up = Closure::wrap(Box::new(move || {
            presser2.set(presser2.get() - 1);
        }) as Box<dyn FnMut()>);
        let mov = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            loc2.set((e.client_x() - left, e.client_y() - top));
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);

        canvas.set_onmousedown(Some(down.as_ref().unchecked_ref()));
        canvas.set_onmouseup(Some(up.as_ref().unchecked_ref()));
        canvas.set_onmousemove(Some(mov.as_ref().unchecked_ref()));

        down.forget();
        mov.forget();
        up.forget();

        // create actual Canvas object
        let mut my_can = Canvas {
            canvas,
            context,
            width,
            height,
            press_count: presser,
            mouse_loc: loc,
        };

        my_can
    }

    pub fn is_mouse_pressed(&self) -> bool {
        self.press_count.get() > 0
    }

    pub fn get_mouse_loc(&self) -> (i32, i32) {
        self.mouse_loc.get()
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
}
