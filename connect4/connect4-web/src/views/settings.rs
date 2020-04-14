use web_sys::MouseEvent;
use yew::prelude::*;
use yew::virtual_dom::VNode;

use crate::components::Menu;
use crate::storage::LocalStorage;
use crate::{console_log, log};

pub struct SettingsPage {
    link: ComponentLink<Self>,
}

#[derive(Debug)]
pub enum Msg {
    ToggleColorBlind,
}

impl Component for SettingsPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        SettingsPage { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        crate::log(&format!("Settings: {:?}", msg));
        match msg {
            Msg::ToggleColorBlind => {
                LocalStorage::set_colorblind_setting(!LocalStorage::get_colorblind_setting());
                true
            }
        }
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let is_colorblind = LocalStorage::get_colorblind_setting();
        html! {
          <Menu topbar="" title="Settings" show_settings=false show_stats=false>
            { toggle_setting("Colorblind Mode", is_colorblind, self.link.callback(|_| Msg::ToggleColorBlind)) }
          </Menu>
        }
    }
}

fn toggle_setting(name: &str, is_on: bool, on_toggle: yew::callback::Callback<MouseEvent>) -> Html {
    let enabled = "bg-green-500";
    let disabled = "bg-red-500";
    let classes = |val| if val { enabled } else { disabled };
    console_log!("I'm {}", classes(is_on));

    let bid = "colourblind-settings-btn";
    let el = crate::document().get_element_by_id(bid);
    if let Some(e) = el {
        e.set_class_name(classes(is_on));
    }
    html! {
        <button id=&bid onclick=on_toggle> { name } </button>
    }
}
