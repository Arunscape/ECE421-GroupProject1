use web_sys::MouseEvent;
use yew::prelude::*;

use crate::components::Menu;
use crate::storage::LocalStorage;

use crate::components::icon;
use crate::components::icon::ConnectIcon;

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
                while let Err(_) = crate::window().location().reload() {}
                true
            }
        }
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
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
    let toggle = icon::html(if is_on {
        ConnectIcon::ToggleOn
    } else {
        ConnectIcon::ToggleOff
    });

    let c = "cursor-pointer flex flex-row bg-blue-500 hover:bg-blue-400 text-white font-bold py-2 px-4 border-b-4 border-blue-700 hover:border-blue-500 rounded text-center my-1 mx-1";
    html! {
        <div onclick=on_toggle class=c>
          <p class="mx-2"> { name } </p>
          { toggle }
        </div>
    }
}
