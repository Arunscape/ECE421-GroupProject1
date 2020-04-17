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
        let player = &self.props.player;
        html! {
            <Menu topbar="" title="Create Game"  show_settings=false show_stats=false>
              <div class="flex flex-col">
                <MenuButton text="Connect4" dest=format!("/finalizegame?game={}&player={}", constants::game::CONNECT4, player)/>
                <MenuButton text="Toot and Otto" dest=format!("/finalizegame?game={}&player={}", constants::game::TOTO, player)/>
                <MenuButton text="Custom Game" dest=format!("/finalizegame?game={}&player={}", constants::game::CUSTOM, player)/>
              </div>
            </Menu>
        }
    }
}
