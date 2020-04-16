use crate::components::{Menu, MenuButton};
use crate::constants;
use yew::{prelude::*, Properties};

pub struct GameConfig {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub player: String,
}

pub enum Msg {}

impl Component for GameConfig {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Menu topbar="" title="Create Game"  show_settings=false show_stats=false>
              <div class="flex flex-col">
                <MenuButton text="Connect4" dest=format!("/setupgame?game={}", constants::game::CONNECT4)/>
                <MenuButton text="Toot and Otto" dest=format!("/setupgame?game={}", constants::game::TOTO)/>
                <MenuButton text="Custom Game" dest=format!("/setupgame?game={}", constants::game::CUSTOM)/>
              </div>
            </Menu>
        }
    }
}
