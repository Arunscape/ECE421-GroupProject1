use web_sys::MouseEvent;
use yew::prelude::*;
use yew::virtual_dom::VNode;

use crate::components::Menu;
use crate::storage::LocalStorage;

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

    fn view(&self) -> VNode {
        let is_colorblind = LocalStorage::get_colorblind_setting();
        html! {
          <Menu topbar="" title="Settings" show_settings=false show_stats=false>
            { toggle_setting("Colorblind Mode", is_colorblind, self.link.callback(|_| Msg::ToggleColorBlind)) }
          </Menu>
        }
    }
}

fn toggle_setting(
    name: &str,
    is_on: bool,
    on_toggle: yew::callback::Callback<MouseEvent>,
) -> VNode {
    let enabled = "bg-green-500";
    let disabled = "bg-red-500";
    let classes = |val| if val { enabled } else { disabled };
    html! {
        <button class={ classes(is_on) } onclick=on_toggle> { name } </button>
    }
}
