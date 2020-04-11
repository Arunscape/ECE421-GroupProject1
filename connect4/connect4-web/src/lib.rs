#![recursion_limit = "1024"]
use connect4_lib::game::Game;
use connect4_lib::games;
use connect4_lib::io::{GameIO, TermIO};

mod canvas;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::initialize();
    web_logger::init();
    App::<Model>::new().mount_to_body();

    let game = games::connect4();
    let c = canvas::Canvas::new("#canvas", 20, 20);

    let mut game = games::connect4_ai();
    yew::run_loop();
    // todo hook up to a button press or something
    connect4_lib::play(&mut game, c);

    Ok(())
}

mod a_component;
mod b_component;
mod c_component;
mod navbar;
mod welcome;
use navbar::Navbar;
use welcome::Welcome;

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
                                AppRoute::Connect4Computer => html!{"Todo, put connect4computer page here"},
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
