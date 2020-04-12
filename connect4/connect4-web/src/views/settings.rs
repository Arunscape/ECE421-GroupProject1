use yew::prelude::*;
use yew::virtual_dom::VNode;

pub struct SettingsPage {}

impl Component for SettingsPage  {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        SettingsPage {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
        html! {
            <p> { "Settings" } </p>
        }
    }
}
