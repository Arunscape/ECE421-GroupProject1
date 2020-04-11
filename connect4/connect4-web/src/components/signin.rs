use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::{prelude::*, virtual_dom::VNode, Properties};
use wasm_bindgen_futures::JsFuture;
use serde::{Serialize, Deserialize};
use yew_router::prelude::*;


pub struct Signin{
    link: ComponentLink<Self>,
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
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> VNode {

        


        html! {
            <button onclick=self.link.callback(|_| Msg::ButtonClick)>{"Click me"}
            </button>
            }
    }
}
