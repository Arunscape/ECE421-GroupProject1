use yew::prelude::*;
use yew::virtual_dom::VNode;

use crate::components::MenuButtonLight;
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
            <div class="h-full flex flex-col items-center justify-between">
                <div class="w-full flex px-5 justify-between">
                  <p></p>
                  <MenuButtonLight text="home" dest="/"/>
                </div>
                <h1 class="font-comic text-6xl">{ "Statistics" }</h1>
                <div>
                  <h2>{format!("Games won: {}", self.stats.games_won)}</h2>
                  <h2>{format!("Games lost: {}", self.stats.games_lost)}</h2>
                  <h2>{format!("Games drawed: {}", self.stats.games_drawed)}</h2>
                  <h2>{format!("Games ongoing: {}", self.stats.games_ongoing)}</h2>
                  <h2>{format!("Games completed: {}", self.stats.games_won + self.stats.games_lost + self.stats.games_drawed)}</h2>
                </div>
                <div class="w-full flex md:justify-end">
                </div>
            </div>
        }
    }
}
