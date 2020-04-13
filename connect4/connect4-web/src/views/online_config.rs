use web_sys::MouseEvent;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::InputData;

use crate::components::router::query;
use crate::components::{Menu, MenuButton};
use crate::storage::LocalStorage;
use crate::window;

pub struct OnlineConfigPage {
    link: ComponentLink<Self>,
    roomcode_text: String,
}

#[derive(Debug)]
pub enum Msg {
    EditRoomCode(String),
    SubmitRoomCode,
    CreateGame,
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
            Msg::SubmitRoomCode => {}
            Msg::CreateGame => {}
        };
        true
    }

    fn view(&self) -> VNode {
        let is_colorblind = LocalStorage::get_colorblind_setting();
        let game = query("game").expect("game not in querystring");
        let url = window().location().href().unwrap();
        let querystring = url
            .split('?')
            .skip(1)
            .next()
            .expect("failed to get querystring");

        html! {
        <Menu title=format!("Online {}", game) topbar="" show_settings=false show_stats=false>
          <div class="flex flex-col">
            <MenuButton text="Create Game" dest=format!("/game/online?{}", querystring)/>
            <p>{"Or, enter a room code to join an existing game"}</p>
            <input placeholder="Room code" type="text" value={&self.roomcode_text} oninput=self.link.callback(|e: InputData| Msg::EditRoomCode(e.value))/>
          </div>
        </Menu>
          }
    }
}
