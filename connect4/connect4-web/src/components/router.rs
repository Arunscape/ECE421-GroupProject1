use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::switch::{AllowMissing, Permissive};
use yew_router::{prelude::*, Switch};

use crate::coms;
use crate::components::{Menu, MenuButton, Signin};
use crate::storage::LocalStorage;
use crate::views::{GameScreen, OnlineConfigPage, SettingsPage, Statistics, ViewPage};
use crate::{constants, window};

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
        coms::sync_refresh();
        true
    }

    fn view(&self) -> VNode {
        html! {
        <Router<AppRoute>
            render = Router::render(|switch: AppRoute| {
                match switch {
                    AppRoute::Settings => html!{<SettingsPage/>},
                    AppRoute::Root => homescreen(),
                    AppRoute::Signin => html!{<Signin/>},
                    AppRoute::NewGame => create_game(),
                    AppRoute::PlayerConfig => player_config(),
                    AppRoute::AIConfig => ai_config(),
                    AppRoute::Game => html!{<GameScreen/>},
                    AppRoute::ScoreBoard => html!{"Todo, put scoreboard page here"},
                    AppRoute::Scores => html!{"Todo, put scores page here"},
                    AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found"},
                    AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)},
                    AppRoute::OnlineConfig => html!{<OnlineConfigPage/>},
                    AppRoute::ViewGames => html!{<ViewPage/>},
                    AppRoute::Statistics => html!{<Statistics/>},
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
    if let Some(_) = LocalStorage::get_username() {
        html! {
            <Menu title="New Game" topbar="" show_settings=false show_stats=false>
              <div class="flex flex-col">
                <MenuButton text="Connect4" dest=format!("/setupgame?game={}", constants::game::CONNECT4)/>
                <MenuButton text="Toot and Otto" dest=format!("/setupgame?game={}", constants::game::TOTO)/>
                <MenuButton text="Custom Game" dest=format!("/setupgame?game={}", constants::game::CUSTOM)/>
              </div>
            </Menu>
        }
    } else {
        html! {
            <Menu title="New Game" topbar="" show_settings=false show_stats=false>
              <div class="flex flex-col">
                <MenuButton text="Connect4" dest=format!("/setupai?game={}", constants::game::CONNECT4)/>
                <MenuButton text="Toot and Otto" dest=format!("/setupai?game={}", constants::game::TOTO)/>
              </div>
            </Menu>
        }
    }
}

fn player_config() -> VNode {
    let preset = query("game").unwrap_or(String::from("connect4"));
    html! {
        <Menu title="Setup Players" topbar=""  show_settings=false show_stats=false>
          <div class="flex flex-col">
            <MenuButton text="Single player" dest=format!("/setupai?game={}", preset)/>
            <MenuButton text="Local Multiplayer" dest=format!("/game/offline?game={}&player={}", preset, constants::player::LOCAL)/>
            <MenuButton text="Online Multiplayer" dest=format!("/setuponline?game={}&player={}", preset, constants::player::REMOTE)/>
          </div>
        </Menu>
    }
}

fn ai_config() -> VNode {
    // TODO: do a nicer selection then just 6 buttons
    // like a radio for dificulty, and a radio for p1/p2
    let preset = query("game").unwrap_or(String::from("connect4"));
    html! {
        <Menu title="Setup AI" topbar=""  show_settings=false show_stats=false>
          <div class="flex flex-col">
            <MenuButton text="Player 1 Easy" dest=format!("/game/offline?game={}&player={}", preset, constants::player::AI_EASY)/>
            <MenuButton text="Player 1 Medium" dest=format!("/game/offline?game={}&player={}", preset, constants::player::AI_MID)/>
            <MenuButton text="Player 1 Hard" dest=format!("/game/offline?game={}&player={}", preset, constants::player::AI_HARD)/>
            <MenuButton text="Player 2 Easy" dest=format!("/game/offline?game={}&player={}", preset, constants::player::AI_EASY2)/>
            <MenuButton text="Player 2 Medium" dest=format!("/game/offline?game={}&player={}", preset, constants::player::AI_MID2)/>
            <MenuButton text="Player 2 Hard" dest=format!("/game/offline?game={}&player={}", preset, constants::player::AI_HARD2)/>
          </div>
        </Menu>
    }
}

fn homescreen() -> VNode {
    if let Some(s) = LocalStorage::get_username() {
        html! {
            <Menu topbar=format!("Hello, {}", s) title="Connecty"  show_settings=false show_stats=true>
              <div class="flex flex-col">
                <MenuButton text="Create Game" dest="/newgame"/>
                <MenuButton text="Current Games" dest="/viewgames/current"/>
                <MenuButton text="Past Games" dest="/viewgames/past"/>
                <MenuButton text="Statistics" dest="/statistics"/>
                <MenuButton text="Settings" dest="/settings"/>
              </div>
            </Menu>
        }
    } else {
        html! {
            <Menu topbar="" title="Connecty"  show_settings=false show_stats=false>
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
    #[to = "/setuponline"]
    OnlineConfig,
    #[to = "/Scores!"]
    Scores,
    #[to = "/settings!"]
    Settings,
    #[to = "/viewgames"]
    ViewGames,
    #[to = "/statistics!"]
    Statistics,
}

pub fn query(key: &str) -> Option<String> {
    let url = window().location().href().unwrap();
    url.split('?')
        .skip(1)
        .next()
        .map(|x| x.split('&'))
        .map(|items| items.filter(|x| x.split('=').next() == Some(key)))
        .and_then(|mut x| x.next())
        .and_then(|x| x.split('=').skip(1).next())
        .map(|x| String::from(x))
}
