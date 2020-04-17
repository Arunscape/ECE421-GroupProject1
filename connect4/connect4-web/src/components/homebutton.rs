use yew::{prelude::*, virtual_dom::VNode};

pub struct HomeButton;

impl Component for HomeButton {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
        html! {
            <a href={ yew::html::Href::from("/") } class="">
              { "Home" }
            </a>
        }
    }
}
