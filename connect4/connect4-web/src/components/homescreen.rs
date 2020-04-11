use crate::window;

use yew::prelude::*;

pub struct HomeComponent {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Start
}

impl Component for HomeComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Start => {
                window().location().set_href("/game");
            }
        };
        true
    }

    fn view(&self) -> Html {
        let start_callback = self.link.callback(|_| Msg::Start);
        html! {
            <div>
                <button onclick=start_callback> { "Start" } </button>
            </div>
        }
    }
}
