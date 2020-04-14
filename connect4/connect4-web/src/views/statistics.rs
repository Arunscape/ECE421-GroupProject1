use web_sys::MouseEvent;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::InputData;

use crate::components::router::query;
use crate::components::{Menu, MenuButton};
use crate::coms;
use crate::constants;
use crate::storage::LocalStorage;
use crate::window;
use connect4_coms::types::GameData;
use connect4_coms::types::GameStats;
use connect4_lib::{game, games};
use wasm_bindgen_futures::spawn_local;

pub struct Statistics {
    link: ComponentLink<Self>,
    stats: GameStats,
}

#[derive(Debug)]
pub enum Msg {
    UpdateStats(GameStats),
}

impl Component for Statistics {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // todo get statistics

        async fn get_stats(link: ComponentLink<Statistics>) {
            let stats = coms::get_player_stats().await;

            let stats = match stats {
                Some(s) => s,
                _ => {
                    crate::alert("Failed to fetch stats. Try refreshing.");
                    GameStats::default()
                }
            };
            link.send_message(Msg::UpdateStats(stats.clone()));
        }
        spawn_local(get_stats(link.clone()));
        let stats = GameStats::default();
        Self { stats, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        crate::log(&format!("Statistics: {:?}", msg));
        match msg {
            Msg::UpdateStats(s) => {
                self.stats = s;
                true
            }
            _ => false,
        }
    }

    fn view(&self) -> VNode {
        html! {
          <div>
            <h2>{format!("Games won: {}", self.stats.games_won)}</h2>
            <h2>{format!("Games lost: {}", self.stats.games_lost)}</h2>
            <h2>{format!("Games drawn: {}", self.stats.games_played - self.stats.games_lost - self.stats.games_lost)}</h2>
            <h2>{format!("Games played: {}", self.stats.games_played)}</h2>
          </div>
        }
    }
}
