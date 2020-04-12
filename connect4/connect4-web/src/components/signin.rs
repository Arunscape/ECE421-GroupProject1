use super::super::window;
use crate::{coms, storage::LocalStorage};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::{prelude::*, virtual_dom::VNode, InputData, Properties};
use yew_router::prelude::*;

pub struct Signin {
    link: ComponentLink<Self>,
    hm: String,
    username: String,
    password: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub enum Msg {
    ButtonClick,
    UpdateUserName(String),
    UpdatePassword(String),
}

// todo delete me
#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub commit: CommitDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitDetails {
    pub author: Signature,
    pub committer: Signature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signature {
    pub name: String,
    pub email: String,
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
        match msg {
            Msg::ButtonClick => {
                self.hm = String::from("Signing in...");

                async fn handleSignin(username: String, password: String) {
                    let token: Option<String> = coms::signin(&username, &password).await;

                    match token {
                        Some(s) => {
                            LocalStorage::set_token(&s);
                            window().location().set_href("/");
                        }
                        None => {}
                    };
                }
                spawn_local(handleSignin(self.username.clone(), self.password.clone()));
            }
            Msg::UpdateUserName(s) => self.username = s,
            Msg::UpdatePassword(s) => self.password = s,
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> VNode {
        let update_username = |e: InputData| Msg::UpdateUserName(e.value);
        let update_password = |e: InputData| Msg::UpdatePassword(e.value);
        let input_class = "mx-3 shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline";
        let but_class = "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline";
        html! {
        <div class="bg-gray-400 h-full flex flex-col justify-around items-center">
          <div class="bg-white shadow  rounded h-64 flex flex-col justify-around items-center">
            <h1 class="font-popper">{"Sign in for more features!"}</h1>
            <div class="flex justify-around">
              <input class={input_class} type="text" value={&self.username} oninput=self.link.callback(update_username) placeholder={"Username"}/>
              <input class={input_class} type="password" value={&self.password} oninput=self.link.callback(update_password) placeholder={"Password"}/>
            </div>
            <button class={but_class} onclick=self.link.callback(|_| Msg::ButtonClick)> { &self.hm } </button>
          </div>
        </div>
        }
    }
}
