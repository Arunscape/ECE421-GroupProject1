use connect4_lib::game::Game;
use connect4_lib::games;
use connect4_lib::io::{GameIO, TermIO};

mod canvas;
mod controller;
mod components;

use crate::components::webio::WebIOComponent;
use crate::components::homescreen::HomeComponent;


use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;


#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<App>();

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

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/game"]
    Connect4Computer,

    #[to = "/"]
    Home,
}

pub struct App {
    link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <Router<AppRoute>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Connect4Computer => html!{<WebIOComponent/>},
                        AppRoute::Home => html!{<HomeComponent/>},
                    }
                })
            />
        }
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
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
        .now() / 1000.0
}
