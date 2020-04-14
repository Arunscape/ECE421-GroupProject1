use web_sys::MouseEvent;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::InputData;

use crate::components::router::query;
use crate::components::{Menu, MenuButton};
use crate::coms;
use crate::constants;
use crate::storage::LocalStorage;
use crate::window;
use connect4_lib::{game, games};
use wasm_bindgen_futures::spawn_local;

pub struct OnlineConfigPage {
    link: ComponentLink<Self>,
    roomcode_text: String,
}

#[derive(Debug)]
pub enum Msg {
    EditRoomCode(String),
    CreateGame,
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
        let url = window().location().href().unwrap();
        let querystring = url
            .split('?')
            .skip(1)
            .next()
            .expect("failed to get querystring");
        async fn get_game(room_code: String, querystring: String) {
            let game = coms::getgame(&room_code).await;
            match game {
                Some(game_data) => {
                    let spots = coms::join_game(&game_data.roomcode).await;
                    if let Some(s) = spots {
                        if !s.iter().any(|x| x.is_none()) {
                            window().location().set_href(&format!(
                                "game/{}?{}",
                                game_data.roomcode.to_string(),
                                querystring.to_string()
                            ));
                        } else {
                            crate::alert("Room is full!");
                        }
                    } else {
                        crate::log("something went horribly wrong");
                        //todo better code style
                    }
                }
                None => crate::log("Invalid room code entered"),
            };
        }

        async fn create_game(querystring: String) {
            let game_type = match query(&"game").unwrap() {
                s if s == constants::game::CONNECT4 => games::GameType::Connect4,
                s if s == constants::game::TOTO => games::GameType::Toto,
                _ => unreachable!(),
            };
            let game =
                games::build_game(game_type, game::PlayerType::Local, game::PlayerType::Remote);
            let game = coms::create_game(game).await;

            match game {
                Some(game_data) => {
                    let spots = coms::join_game(&game_data.roomcode).await;
                    if let Some(s) = spots {
                        if !s.iter().any(|x| x.is_none()) {
                            window().location().set_href(&format!(
                                "game/{}?{}",
                                game_data.roomcode.to_string(),
                                querystring.to_string()
                            ));
                        } else {
                            crate::alert("Room is full!");
                        }
                    } else {
                        crate::log("something went horribly wrong");
                        //todo better code style
                    }
                }
                None => crate::alert("Invalid room code entered"),
            };
        }

        match msg {
            Msg::EditRoomCode(s) => self.roomcode_text = s,
            Msg::CreateGame => spawn_local(create_game(querystring.to_string())),
            Msg::SubmitRoomCode => spawn_local(get_game(
                self.roomcode_text.to_string(),
                querystring.to_string(),
            )),
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
            <button onclick=self.link.callback(|_| Msg::CreateGame)>{"Create Game"}</button>
            <p>{"Or, enter a room code to join an existing game"}</p>
            <input placeholder="Room code" type="text" value={&self.roomcode_text} oninput=self.link.callback(|e: InputData| Msg::EditRoomCode(e.value))/>
            <button onclick=self.link.callback(|_| Msg::SubmitRoomCode )>{"Submit Room Code"}</button>
          </div>
        </Menu>
          }
    }
}
