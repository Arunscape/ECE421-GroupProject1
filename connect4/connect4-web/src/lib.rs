#![recursion_limit = "1024"]
#![feature(async_closure)]

use connect4_lib::game::Game;
use connect4_lib::games;
use connect4_lib::io::{GameIO, TermIO};

mod canvas;
mod jq;
mod components;
mod coms;
mod game_object;
mod constants;
mod controller;
mod storage;
mod views;

use crate::components::router::ConnectRouter;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    log("Starting Yew");
    coms::test_request();
    yew::initialize();
    web_logger::init();
    App::<ConnectRouter>::new().mount_to_body();

    yew::run_loop();

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn set_timeout(f: &Closure<dyn FnMut()>, millis: i32) {
    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(f.as_ref().unchecked_ref(), millis)
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

fn seconds() -> f64 {
    window()
        .performance()
        .expect("performance should be available")
        .now()
        / 1000.0
}
