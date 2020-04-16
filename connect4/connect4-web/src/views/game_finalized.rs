use yew::prelude::*;
// use crate::constants;

pub struct GameFinalized;

impl Component for GameFinalized {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
    fn mounted(&mut self) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <p> { "Loading..." } </p>
        }
    }
}
