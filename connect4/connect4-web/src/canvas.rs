use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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
        let my_can = Canvas { canvas, context };

        my_can
    }

    pub fn draw_circle(&self, x: f64, y: f64, r: f64, fill: String, stroke: String) {
        self.context.save();
        self.context.set_fill_style(&fill.into());
        self.context.set_stroke_style(&stroke.into());
        self.context.begin_path();
        self.context
            .arc(x, y, r, 0.0, 2.0 * std::f64::consts::PI)
            .unwrap();
        self.context.fill();
        self.context.restore();
    }

    pub fn register_onclick_listener(&self, onclick: Box<dyn FnMut(web_sys::MouseEvent)>) {
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

    pub fn get_width(&self) -> f64 {
        self.canvas.width() as f64
    }

    pub fn get_height(&self) -> f64 {
        self.canvas.height() as f64
    }

    pub fn is_skinny(&self) -> bool {
        self.get_height() * 5.0 > self.get_width() * 4.0
    }
}
