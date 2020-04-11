#![recursion_limit = "1024"]
#![feature(async_closure)]

use connect4_lib::game::Game;
use connect4_lib::games;
use connect4_lib::io::{GameIO, TermIO};

mod components;
mod coms;
mod controller;
use crate::components::webio::WebIOComponent;
use components::{a_component, b_component, c_component};
mod storage;

use components::{navbar::Navbar, signin::Signin, welcome::Welcome};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    log("Starting Yew");
    coms::test_request();
    yew::initialize();
    web_logger::init();
    App::<Model>::new().mount_to_body();

    yew::run_loop();

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
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
        .now()
        / 1000.0
}

use yew::prelude::*;

use yew_router::{prelude::*, Switch};

use crate::{
    a_component::AModel,
    b_component::{BModel, BRoute},
    c_component::CModel,
};
use yew::virtual_dom::VNode;
use yew_router::switch::{AllowMissing, Permissive};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
        html! {
            <div>
                <Navbar/>
                <div style="margin-left:390px;margin-right:40px">
                // TODO remove these examples
                <nav class="menu",>
                    <RouterButton<AppRoute> route=AppRoute::A(AllowMissing(None))> {"Go to A"} </RouterButton<AppRoute>>
                    <RouterAnchor<AppRoute> route=AppRoute::B(BRoute::None)> {"Go to B"} </RouterAnchor<AppRoute>>
                    <RouterButton<AppRoute> route=AppRoute::C> {"Go to C"} </RouterButton<AppRoute>>
                    <RouterButton<AppRoute> route=AppRoute::A(AllowMissing(Some(ARoute)))> {"Go to A/C"} </RouterButton<AppRoute>>
                    <RouterButton<AppRoute> route=AppRoute::E("there".to_string())> {"Go to E (hello there)"} </RouterButton<AppRoute>>
                    <RouterButton<AppRoute> route=AppRoute::E("world".to_string())> {"Go to E (hello world)"} </RouterButton<AppRoute>>
                    <RouterButton<AppRoute> route=AppRoute::PageNotFound(Permissive(Some("nope".to_string())))> {"Go to bad path"} </RouterButton<AppRoute>>
                </nav>
                    <Router<AppRoute>
                        render = Router::render(|switch: AppRoute| {
                            match switch {
                                AppRoute::Root => html!{<Welcome/>},
                                AppRoute::Signin => html!{<Signin/>},
                                AppRoute::A(AllowMissing(route)) => html!{<AModel route = route />},
                                AppRoute::B(route) => {
                                    let route: b_component::Props = route.into();
                                    html!{<BModel with route/>}
                                },
                                AppRoute::C => html!{<CModel />},
                                AppRoute::E(string) => html!{format!("hello {}", string)},
                                AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found"},
                                AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)},
                                AppRoute::HowToConnect4 => html!{"Todo, put howtoconnect4 page here"},
                                AppRoute::Connect4Computer => html!{<WebIOComponent/>},
                                AppRoute::Connect4Human => html!{"Todo, put connect4human page here"},
                                AppRoute::HowToToot => html!{"Todo, put howtotoot page here"},
                                AppRoute::TootOttoComputer => html!{"Todo, put tootcomputer page here"},
                                AppRoute::TootOttoHuman => html!{"Todo, put toothuman page here"},
                                AppRoute::ScoreBoard => html!{"Todo, put scoreboard page here"},
                                AppRoute::Scores => html!{"Todo, put scores page here"},
                            }
                        })
                        redirect = Router::redirect(|route: Route| {
                            AppRoute::PageNotFound(Permissive(Some(route.route)))
                        })
                    />
                </div>
            </div>
        }
    }
}

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/!"]
    Root,
    #[to = "/signin!"]
    Signin,
    #[to = "/a{*:inner}"]
    A(AllowMissing<ARoute>),
    #[to = "/b{*:inner}"]
    B(BRoute),
    #[to = "/c!"]
    C,
    #[to = "/e/{string}"]
    E(String),
    #[to = "/page-not-found!"]
    PageNotFound(Permissive<String>),
    #[to = "/HowToConnect4!"]
    HowToConnect4,
    #[to = "/Connect4Computer!"]
    Connect4Computer,
    #[to = "/Connect4Human!"]
    Connect4Human,
    #[to = "/HowToToot!"]
    HowToToot,
    #[to = "/TootOttoComputer!"]
    TootOttoComputer,
    #[to = "/TootOttoHuman!"]
    TootOttoHuman,
    #[to = "/ScoreBoard!"]
    ScoreBoard,
    #[to = "/Scores!"]
    Scores,
}

#[derive(Debug, Switch, PartialEq, Clone, Copy)]
#[to = "/c"]
pub struct ARoute;
