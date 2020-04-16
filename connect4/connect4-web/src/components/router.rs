use yew::prelude::*;
use yew::virtual_dom::{VList, VNode};
use yew_router::switch::{AllowMissing, Permissive};
use yew_router::{prelude::*, Switch};

use crate::components::{GameConfig, Signin}; // TODO: move these to views
use crate::components::{Menu, MenuButton};
use crate::coms;
use crate::storage::LocalStorage;
use crate::views::{
    GameFinalized, GameScreen, OnlineConfigPage, SettingsPage, Statistics, ViewPage,
};
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
                    AppRoute::NewGame =>  player_config(),
                    AppRoute::GameConfig => game_config(),
                    AppRoute::AIConfig => ai_config(),
                    AppRoute::Game => html!{<GameScreen/>},
                    AppRoute::ScoreBoard => html!{"Todo, put scoreboard page here"},
                    AppRoute::Scores => html!{"Todo, put scores page here"},
                    AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found"},
                    AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)},
                    AppRoute::OnlineConfig => html!{<OnlineConfigPage/>},
                    AppRoute::ViewGames => html!{<ViewPage/>},
                    AppRoute::Statistics => html!{<Statistics/>},
                    AppRoute::Finalize => html!{<GameFinalized/>},
                }
            })

                redirect = Router::redirect(|route: Route| {
                    AppRoute::PageNotFound(Permissive(Some(route.route)))
                })
            />
        }
    }
}

fn game_config() -> Html {
    html! {<GameConfig player=query("player").unwrap_or(String::from("local"))/>}
}

fn player_config() -> VNode {
    html! {
        <Menu title="Setup Players" topbar=""  show_settings=false show_stats=false>
          <div class="flex flex-col">
            <MenuButton text="Single player" dest=format!("/setupai")/>
            <MenuButton text="Local Multiplayer"
                        dest=format!("/setupgame&player={}", constants::player::LOCAL)/>
            { render_if( html! {<MenuButton text="Online Multiplayer" dest=format!("/setuponline")/>}, LocalStorage::get_username().is_some()) }
          </div>
        </Menu>
    }
}

fn ai_config() -> VNode {
    // TODO: do a nicer selection then just 6 buttons
    // like a radio for dificulty, and a radio for p1/p2
    html! {
        <Menu title="Setup AI" topbar=""  show_settings=false show_stats=false>
          <div class="flex flex-col">
            <MenuButton text="Player 1 Easy" dest=format!("/setupgame?player={}", constants::player::AI_EASY)/>
            <MenuButton text="Player 1 Medium" dest=format!("/setupgame?player={}", constants::player::AI_MID)/>
            <MenuButton text="Player 1 Hard" dest=format!("/setupgame?player={}", constants::player::AI_HARD)/>
            <MenuButton text="Player 2 Easy" dest=format!("/setupgame?player={}", constants::player::AI_EASY2)/>
            <MenuButton text="Player 2 Medium" dest=format!("/setupgame?player={}", constants::player::AI_MID2)/>
            <MenuButton text="Player 2 Hard" dest=format!("/setupgame?player={}", constants::player::AI_HARD2)/>
          </div>
        </Menu>
    }
}

fn homescreen() -> VNode {
    if let Some(s) = LocalStorage::get_username() {
        html! {
            <Menu topbar=format!("Hello, {}", s) title="Connecty"  show_settings=true show_stats=true>
              <div class="flex flex-col">
                <MenuButton text="Create Game" dest="/newgame"/>
                <MenuButton text="Current Games" dest="/viewgames/current"/>
                <MenuButton text="Past Games" dest="/viewgames/past"/>
              </div>
            </Menu>
        }
    } else {
        html! {
            <Menu topbar="" title="Connecty"  show_settings=true show_stats=false>
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
    GameConfig,
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
    #[to = "/finalizegame"]
    Finalize,
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

pub fn render_if(render: VNode, condition: bool) -> VNode {
    if condition {
        render
    } else {
        VNode::from(VList::new())
    }
}
