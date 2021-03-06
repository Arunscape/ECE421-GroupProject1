use super::super::window;
use crate::{coms, storage::LocalStorage};

use wasm_bindgen_futures::spawn_local;

use yew::{prelude::*, virtual_dom::VNode, InputData, Properties};

pub struct Signin {
    link: ComponentLink<Self>,
    hm: String,
    username: String,
    password: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

#[derive(Debug)]
pub enum Msg {
    ButtonClick,
    UpdateUserName(String),
    UpdatePassword(String),
    UpdateMessage(String),
}

impl Component for Signin {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            hm: String::from("Sign in"),
            username: String::new(),
            password: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        crate::log(&format!("Recived Message: {:?}", msg));
        match msg {
            Msg::ButtonClick => {
                self.link
                    .send_message(Msg::UpdateMessage(String::from("Signing in...")));

                async fn handle_signin(
                    username: String,
                    password: String,
                    msg: ComponentLink<Signin>,
                ) {
                    let token: Option<String> = coms::signin(&username, &password).await;

                    crate::log(&format!("Recived Token: {:?}", token));
                    match token {
                        Some(s) => {
                            if s == "" {
                                crate::log(&format!("Sending Callback 1"));
                                msg.send_message(Msg::UpdateMessage(String::from(
                                    "Incorrect Password",
                                )));
                            } else {
                                LocalStorage::set_token(&s);
                                window()
                                    .location()
                                    .set_href("/")
                                    .expect("failed to set redirect to root");
                            }
                        }
                        None => {
                            msg.send_message(Msg::UpdateMessage(String::from("Error")));
                        }
                    };
                }
                spawn_local(handle_signin(
                    self.username.clone(),
                    self.password.clone(),
                    self.link.clone(),
                ));
            }
            Msg::UpdateUserName(s) => self.username = s,
            Msg::UpdatePassword(s) => self.password = s,
            Msg::UpdateMessage(s) => {
                crate::log(&format!("Recived msg: {}", s));
                self.hm = s
            }
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> VNode {
        let update_username = |e: InputData| Msg::UpdateUserName(e.value);
        let update_password = |e: InputData| Msg::UpdatePassword(e.value);
        let input_class = "mx-3 shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline mx-3 my-3";
        let but_class = "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline";
        html! {
        <div class="bg-gray-400 h-full flex flex-col justify-around items-center">
          <div class="bg-white shadow  rounded h-64 flex flex-col justify-around items-center">
            <h1 class="font-popper">{"Sign in for more features!"}</h1>
            <div class="flex flex-col justify-center items-center px-4">
              <input class={input_class} type="text" value={&self.username} oninput=self.link.callback(update_username) placeholder={"Username"}/>
              <input class={input_class} type="password" value={&self.password} oninput=self.link.callback(update_password) placeholder={"Password"}/>
            </div>
            <button class={but_class} onclick=self.link.callback(|_| Msg::ButtonClick)> { &self.hm } </button>
          </div>
        </div>
        }
    }
}
