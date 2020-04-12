use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::{prelude::*, Switch};
use yew_router::switch::{AllowMissing, Permissive};

use crate::window;
use crate::storage::LocalStorage;
use crate::components::{WebIOComponent, Signin, Menu, MenuButton};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct ConnectRouter {}

impl Component for ConnectRouter {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ConnectRouter {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn mounted(&mut self) -> ShouldRender {
        // if let None = LocalStorage::get_token() {
        //     if window().location().pathname().unwrap() != "/signin" {
        //         window().location().set_href("/signin");
        //     }
        // }
        true
    }

    fn view(&self) -> VNode {
        html! {
            <Router<AppRoute>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Root => homescreen(),
                        AppRoute::Signin => html!{<Signin/>},
                        AppRoute::Game => html!{<WebIOComponent/>},
                        AppRoute::ScoreBoard => html!{"Todo, put scoreboard page here"},
                        AppRoute::Scores => html!{"Todo, put scores page here"},
                        AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found"},
                        AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)},
                    }
                })

                redirect = Router::redirect(|route: Route| {
                    AppRoute::PageNotFound(Permissive(Some(route.route)))
                })
            />
        }
    }
}

fn homescreen() -> VNode {
   if let Some(s) = LocalStorage::get_token() {
      html!{
          <Menu topbar="Hello, {username}" show_sound=false show_settings=false show_stats=true>
            <div class="flex flex-col">
              <MenuButton text="Create Game" dest="/game/offline"/>
              <MenuButton text="Current Games" dest="#"/>
              <MenuButton text="Past Games" dest="#"/>
            </div>
          </Menu>
      }
    } else {
      html!{
          <Menu topbar="" show_sound=false show_settings=false show_stats=false>
            <div class="flex flex-col">
              <MenuButton text="Sign In" dest="/signin"/>
              <MenuButton text="Play Offline" dest="/game/offline"/>
            </div>
          </Menu>
      }
    }
}

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/!"]
    Root,
    #[to = "/signin!"]
    Signin,
    #[to = "/page-not-found!"]
    PageNotFound(Permissive<String>),
    #[to = "/ScoreBoard!"]
    ScoreBoard,
    #[to = "/game"]
    Game,
    #[to = "/Scores!"]
    Scores,
}
