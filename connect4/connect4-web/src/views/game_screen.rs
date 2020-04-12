use yew::prelude::*;
use crate::{components::GameComponent, game_object::GameObject};
use crate::router::query;

pub struct GameScreen {
    link: ComponentLink<Self>,
}

pub enum Msg {
    GoBack,
}

impl GameScreen {
    // I OWN THE GAME COMPONENT
    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {  }
    }
    fn update(&mut _self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view() -> VNode {
        
        let game_id = query("game").expect("game id not present in query string");
        let game_object = c
        let active = 
        html! {
            <GameComponent/>
        }
    }
}




