use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use connect4_coms::types::GameData;
use crate::coms;
use crate::components::GameComponent;

pub struct ViewPage {
    link: ComponentLink<Self>,
    games: Vec<GameData>,

}

impl Component for ViewPage {
    type Message = Vec<GameData>;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ViewPage {
            games: Vec::new(),
            link
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.get_data_from_server();
        false
    }

    fn update(&mut self, msg: Vec<GameData>) -> ShouldRender {
        self.games = msg;
        true
    }

    fn view(&self) -> Html {
        let list_styles = "container mx-auto flex flex-col";
        html! {
            <div>
              <h1>{"View"}</h1>
              <ul class={list_styles}>{ self.games.iter().map(renderGameView).collect::<Html>() }</ul>
            </div>
        }
    }
}

fn renderGameView(game: &GameData) -> Html {
    let game_style="";
        html! {
          <div class={game_style}>
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
            link.send_message(data);
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
    let val = url.split('?')
        .next()
        .and_then(|x| x.split('/').skip(4).next())
        .map(|x| String::from(x));
    if let Some(s) = val {
        if s == String::from("current") {
            return PastPresent::Present
        }
    }
    PastPresent::Past
}
