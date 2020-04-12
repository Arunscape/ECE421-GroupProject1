use web_sys::MouseEvent;
use yew::prelude::*;
use yew::virtual_dom::VNode;

use crate::components::Menu;

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
        false
    }

    fn view(&self) -> VNode {
        html! {
          <Menu topbar="" title="Settings" show_settings=false show_stats=false>
            { toggle_setting("Colorblind Mode", false, self.link.callback(|_| Msg::ToggleColorBlind)) }
          </Menu>
        }
    }
}

fn toggle_setting(
    name: &str,
    is_on: bool,
    on_toggle: yew::callback::Callback<MouseEvent>,
) -> VNode {
    html! {
        <button onclick=on_toggle> { name } </button>
    }
}
