use yew::prelude::*;
use crate::{
    components::GameComponent, 
    game_object::GameObject
};
use crate::router::query;

pub struct GameScreen {
    link: ComponentLink<Self>,
}


pub enum Msg {
    GoBack,
}

impl GameScreen {
    // I OWN THE GAME COMPONENT
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {  props }
    }
    fn update(&mut _self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view() -> VNode {
        
        let game_type = query("game").expect("game type not present in query string");
        let other_player = query("player").expect("player not present in query string")
        let active = true;
        html! {
            <GameComponent gameid other_player active/>
        }
    }
}




