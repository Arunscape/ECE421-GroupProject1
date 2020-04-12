use yew::prelude::*;
use crate::{
    components::GameComponent, 
    game_object::GameObject
};
use crate::router::query;

pub struct GameScreen {
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub active: bool,
    pub canvas: Canvas,
    pub game: Game,
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
        
        let game_id = query("game").expect("game id not present in query string");
        let game_object = GameObject::new(self.props.canvas, self.props.game);
        let active = self.props.active;
        html! {
            <GameComponent/>
        }
    }
}




