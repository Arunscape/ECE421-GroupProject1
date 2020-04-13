use yew::prelude::*;
use crate::constants;

use crate::{
    components::GameComponent,
    game_object::GameObject
};

use crate::components::router::query;

pub struct GameScreen {
    link: ComponentLink<Self>,
}

impl Component for GameScreen {
    type Message = ();
    type Properties = ();
    // I OWN THE GAME COMPONENT
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {  link }
    }
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let gt: String = query("game").unwrap_or(String::from("connect4"));
        let op: String = query("player").unwrap_or(String::from("local"));
        //    active=true
        html! {
            <GameComponent game_type=gt other_player=op gameid=String::from("offline") active=true/>
        }
    }
}
