use yew::prelude::*;
use yew::virtual_dom::VNode;

use crate::components::Menu;
use crate::coms;
use connect4_coms::types::GameStats;
use wasm_bindgen_futures::spawn_local;

pub struct Statistics {
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
        Self { stats }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        crate::log(&format!("Statistics: {:?}", msg));
        match msg {
            Msg::UpdateStats(s) => {
                self.stats = s;
                true
            }
        }
    }

    fn view(&self) -> VNode {
        html! {
            <Menu title=format!("Statistics") topbar="" show_settings=false show_stats=false>
              <div>
                <h2>{format!("Games won: {}", self.stats.games_won)}</h2>
                <h2>{format!("Games lost: {}", self.stats.games_lost)}</h2>
                <h2>{format!("Games drawed: {}", self.stats.games_drawed)}</h2>
                <h2>{format!("Games ongoing: {}", self.stats.games_ongoing)}</h2>
                <h2>{format!("Games completed: {}", self.stats.games_won + self.stats.games_lost + self.stats.games_drawed)}</h2>
              </div>
            </Menu>
        }
    }
}
