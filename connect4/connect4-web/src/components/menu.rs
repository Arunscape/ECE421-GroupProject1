use crate::components::router::render_if;
use yew::{prelude::*, virtual_dom::VNode, Properties};

use crate::components::icon;
use crate::components::icon::ConnectIcon;
use crate::components::{MenuButton, MenuButtonLight};

pub struct Menu {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub topbar: String,
    pub title: String,
    pub show_stats: bool,
    pub show_settings: bool,
    pub children: Children,
}

pub enum Msg {
    Msg,
}

impl Component for Menu {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let welcome_style = "text-2xl text-left";
        html! {
            <div class="h-full flex flex-col items-center justify-between">
                <div class="w-full flex px-5 justify-between">
                  <p class=welcome_style> { &self.props.topbar } </p>
                  { signout_or_home(&self.props.title == "Connecty") }
                </div>
                <h1 class="font-comic text-6xl">{ &self.props.title }</h1>
                <div>
                    { self.props.children.render() }
                </div>
                <div class="w-full flex md:justify-end">
                  <div class="w-full flex md:w-32 justify-around">
                    { render_if(html!{icon(ConnectIcon::Stats, "/statistics")}, self.props.show_stats) }
                    { render_if(html!{icon(ConnectIcon::Settings, "/settings")}, self.props.show_settings) }
                  </div>
                </div>
            </div>
        }
    }
}

fn signout() {
    crate::storage::LocalStorage::clear_token();
    crate::window().location().set_href("/");
}

fn signout_or_home(is_home: bool) -> Html {
    let c = crate::components::menubutton::MENU_LIGHT_CLASSES;
    if is_home {
        render_if(
            html! {<button class=c onclick={Callback::from(|_| signout())}> { "Sign out" } </button>},
            crate::storage::LocalStorage::get_token().is_some(),
        )
    } else {
        html! {
          <MenuButtonLight text="home" dest="/"/>
        }
    }
}

fn icon(i: ConnectIcon, dest: &str) -> VNode {
    html! {
      <a href={ yew::html::Href::from(dest)}> { icon::html(i) } </a>
    }
}
