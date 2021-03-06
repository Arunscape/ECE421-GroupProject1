use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::InputData;

use crate::components::router::query;
use crate::components::{Menu, MenuButton};
use crate::game_manager;
pub struct OnlineConfigPage {
    link: ComponentLink<Self>,
    roomcode_text: String,
}

#[derive(Debug)]
pub enum Msg {
    EditRoomCode(String),
    SubmitRoomCode,
}

impl Component for OnlineConfigPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            roomcode_text: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        crate::log(&format!("OnlineConfig: {:?}", msg));

        match msg {
            Msg::EditRoomCode(s) => self.roomcode_text = s,
            Msg::SubmitRoomCode => game_manager::join_game_and_go(self.roomcode_text.to_string()),
        };
        true
    }

    fn view(&self) -> VNode {
        let game = query("game").unwrap_or(String::from("connect4"));
        html! {
        <Menu title=format!("Online {}", game) topbar="" show_settings=false show_stats=false>
          <div class="flex flex-col justify-center text-center">
            <MenuButton text="Create Game" dest="/setupgame?player=remote" />
            <p>{"Or, enter a room code to join an existing game"}</p>
            <input placeholder="Room code" type="text" value={&self.roomcode_text} oninput=self.link.callback(|e: InputData| Msg::EditRoomCode(e.value))/>
            <button onclick=self.link.callback(|_| Msg::SubmitRoomCode )>{"Submit Room Code"}</button>
          </div>
        </Menu>
          }
    }
}

// Msg::CreateGame => game_manager::create_game_and_go(game_manager::create_game(
//     query("game").unwrap_or(String::from("connect4")),
// )),
