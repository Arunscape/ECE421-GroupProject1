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
        true
    }

    fn view(&self) -> VNode {
        html! {
            <Router<AppRoute>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Root => homescreen(),
                        AppRoute::Signin => html!{<Signin/>},
                        AppRoute::NewGame => create_game(),
                        AppRoute::PlayerConfig => player_config(),
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

fn create_game() -> VNode {
      html!{
          <Menu title="New Game" topbar="" show_sound=false show_settings=false show_stats=false>
            <div class="flex flex-col">
              <MenuButton text="Connect4" dest="/setupgame?preset=connect4"/>
              <MenuButton text="Toot and Otto" dest="/setupgame?preset=toto"/>
              <MenuButton text="Custom Game" dest="/setupgame?preset=custom"/>
            </div>
          </Menu>
      }
}

fn player_config() -> VNode {
      html!{
          <Menu title="Setup Players" topbar="" show_sound=false show_settings=false show_stats=false>
            <div class="flex flex-col">
              <MenuButton text="Single player" dest="/game/offline"/>
              <MenuButton text="Local Multiplayer" dest="/game/offline"/>
              <MenuButton text="Online Multiplayer" dest="/game/offline"/>
            </div>
          </Menu>
      }
}

fn homescreen() -> VNode {
   if let Some(s) = LocalStorage::get_username() {
      html!{
          <Menu topbar=format!("Hello, {}", s) title="Connecty" show_sound=false show_settings=false show_stats=true>
            <div class="flex flex-col">
              <MenuButton text="Create Game" dest="/newgame"/>
              <MenuButton text="Current Games" dest="#"/>
              <MenuButton text="Past Games" dest="#"/>
            </div>
          </Menu>
      }
    } else {
      html!{
          <Menu topbar="" title="Connecty" show_sound=false show_settings=false show_stats=false>
            <div class="flex flex-col">
              <MenuButton text="Sign In" dest="/signin"/>
              <MenuButton text="Play Offline" dest="/newgame"/>
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
    #[to = "/newgame"]
    NewGame,
    #[to = "/setupgame"]
    PlayerConfig,
    #[to = "/Scores!"]
    Scores,
}
