use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::{prelude::*, virtual_dom::VNode, Properties};
use wasm_bindgen_futures::JsFuture;
use serde::{Serialize, Deserialize};
use yew_router::prelude::*;
use wasm_bindgen::JsValue;
use super::super::coms;


pub struct Signin{
    link: ComponentLink<Self>,
    hm: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub enum Msg {
    ButtonClick,
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
            hm: String::from("click me!"),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ButtonClick => {
                coms::test_request();
                self.hm = String::from("I was clicked!");
                true
            },
            _ => false,
        }
    }
    

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> VNode {

        


        html! {
            <div>
            <h1>{"/Signin"}</h1>
            <button onclick=self.link.callback(|_| Msg::ButtonClick)>{self.hm.to_string()}
            </button>
            </div>
            }
    }
}
