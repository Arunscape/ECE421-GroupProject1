use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::switch::{AllowMissing, Permissive};
use yew_router::{prelude::*, Switch};

use crate::components::{Menu, MenuButton, Signin, WebIOComponent};
use crate::storage::LocalStorage;
use crate::window;

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
                    AppRoute::AIConfig => ai_config(),
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
  html! {
      <Menu title="New Game" topbar="" show_sound=false show_settings=false show_stats=false>
        <div class="flex flex-col">
          <MenuButton text="Connect4" dest="/setupgame?game=connect4"/>
          <MenuButton text="Toot and Otto" dest="/setupgame?game=toto"/>
          <MenuButton text="Custom Game" dest="/setupgame?game=custom"/>
        </div>
      </Menu>
  }
}

fn player_config() -> VNode {
  let preset = query("game").unwrap_or(String::from("connect4"));
  html! {
      <Menu title="Setup Players" topbar="" show_sound=false show_settings=false show_stats=false>
        <div class="flex flex-col">
          <MenuButton text="Single player" dest=format!("/setupai?game={}", preset)/>
          <MenuButton text="Local Multiplayer" dest=format!("/game/offline?game={}&player=local", preset)/>
          <MenuButton text="Online Multiplayer" dest=format!("/game/offline?game={}&player=remote", preset)/>
        </div>
      </Menu>
  }
}

fn ai_config() -> VNode {
  // TODO: do a nicer selection then just 6 buttons
  // like a radio for dificulty, and a radio for p1/p2
  let preset = query("game").unwrap_or(String::from("connect4"));
  html! {
      <Menu title="Setup AI" topbar="" show_sound=false show_settings=false show_stats=false>
        <div class="flex flex-col">
          <MenuButton text="Player 1 Easy" dest=format!("/game/offline?game={}&player=ai_easy", preset)/>
          <MenuButton text="Player 1 Medium" dest=format!("/game/offline?game={}&player=ai_mid", preset)/>
          <MenuButton text="Player 1 Hard" dest=format!("/game/offline?game={}&player=ai_hard", preset)/>
          <MenuButton text="Player 2 Easy" dest=format!("/game/offline?game={}&player=ai_easy2", preset)/>
          <MenuButton text="Player 2 Medium" dest=format!("/game/offline?game={}&player=ai_mid2", preset)/>
          <MenuButton text="Player 2 Hard" dest=format!("/game/offline?game={}&player=ai_hard2", preset)/>
        </div>
      </Menu>
  }
}

const AI_EASY: &'static str = "ai_easy";
const AI_MID: &'static str = "ai_mid";
const AI_HARD: &'static str = "ai_hard";
const AI_EASY2: &'static str = "ai_easy2";
const AI_MID2: &'static str = "ai_mid2";
const AI_HARD: &'static str = + "ai_hard2";

fn homescreen() -> VNode {
  if let Some(s) = LocalStorage::get_username() {
    html! {
        <Menu topbar=format!("Hello, {}", s) title="Connecty" show_sound=false show_settings=false show_stats=true>
          <div class="flex flex-col">
            <MenuButton text="Create Game" dest="/newgame"/>
            <MenuButton text="Current Games" dest="#"/>
            <MenuButton text="Past Games" dest="#"/>
          </div>
        </Menu>
    }
  } else {
    html! {
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
  #[to = "/setupai"]
  AIConfig,
  #[to = "/Scores!"]
  Scores,
}

pub fn query(key: &str) -> Option<String> {
  let url = window().location().href().unwrap();
  url
    .split('?')
    .skip(1)
    .next()
    .map(|x| x.split('&'))
    .map(|items| items.filter(|x| x.split('=').next() == Some(key)))
    .and_then(|mut x| x.next())
    .and_then(|x| x.split('=').skip(1).next())
    .map(|x| String::from(x))
}
