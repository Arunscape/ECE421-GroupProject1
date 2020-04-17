use crate::components::GameComponent;
use crate::coms;

use connect4_coms::types::GameData;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

pub struct ViewPage {
    link: ComponentLink<Self>,
    games: Vec<GameData>,
}

pub enum Msg {
    Server(Vec<GameData>),
    Link(usize),
}

impl Component for ViewPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ViewPage {
            games: Vec::new(),
            link,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.get_data_from_server();
        false
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::Server(data) => self.games = data,
            Msg::Link(i) => {
                let s = self.games[i].roomcode.clone();
                crate::window()
                    .location()
                    .set_href(&format!("/game/{}", s))
                    .expect("unable to redirect to /game");
            }
        }
        true
    }

    fn view(&self) -> Html {
        let list_styles = "container mx-auto flex flex-col";
        html! {
            <div>
              <h1>{"View"}</h1>
              <ul class={list_styles}>{
                  self.games.iter().enumerate()
                      .map(|(i, x)| render_game_view(x, i, self.link.clone())).collect::<Html>()
              }</ul>
            </div>
        }
    }
}

fn render_game_view(game: &GameData, index: usize, link: ComponentLink<ViewPage>) -> Html {
    let game_style = "";

    let id = index;
    let onclick = link.callback(move |_| Msg::Link(id));
    html! {
      <div onclick=onclick class={game_style}>
        <GameComponent gameid=game.roomcode.clone()
            other_player=String::from("")
            game_type=String::from("")
            active=false/>
      </div>
    }
}

impl ViewPage {
    fn get_data_from_server(&self) {
        async fn asyncr(link: ComponentLink<ViewPage>) {
            let data = match get_past_present() {
                PastPresent::Present => coms::getgamespresent().await,
                PastPresent::Past => coms::getgamespast().await,
            };
            link.send_message(Msg::Server(data));
        }
        spawn_local(asyncr(self.link.clone()));
    }
}

enum PastPresent {
    Past,
    Present,
}

fn get_past_present() -> PastPresent {
    let url = crate::window().location().href().unwrap();
    let val = url
        .split('?')
        .next()
        .and_then(|x| x.split('/').skip(4).next())
        .map(|x| String::from(x));
    if let Some(s) = val {
        if s == String::from("current") {
            return PastPresent::Present;
        }
    }
    PastPresent::Past
}
