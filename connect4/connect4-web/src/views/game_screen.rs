use crate::constants;
use yew::prelude::*;

use crate::{components::GameComponent, game_object::GameObject};

use crate::components::router::query;

use crate::{console_log, log};

pub struct GameScreen {
    link: ComponentLink<Self>,
}

impl Component for GameScreen {
    type Message = ();
    type Properties = ();
    // I OWN THE GAME COMPONENT
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let gt = query("game").unwrap_or(String::from("connect4"));
        let op = query("player").unwrap_or(String::from("local"));
        let id = get_game_code();
        console_log!("Creating game component for room: {}", id);
        //    active=true
        html! {
            <GameComponent game_type=gt other_player=op gameid=id active=true/>
        }
    }
}

fn get_game_code() -> String {
    let url = crate::window().location().href().unwrap();
    url.split('?')
        .next()
        .and_then(|x| x.split('/').skip(4).next())
        .map(|x| String::from(x))
        .unwrap_or(String::from("offline"))
}
