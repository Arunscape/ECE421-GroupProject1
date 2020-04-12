use yew::prelude::*;

pub struct GameComponent {
    link: ComponentLink<Self>,
}

pub enum Msg {
    GoBack,
}

impl GameComponent {
    // I OWN THE GAME OBJECT AND DECIDE WHAT TO DO WITH THE PROPS I RECEIVE
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }
    fn update(&mut _self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view() -> VNode {
        html! {
            <canvas classes="h-full w-full" id="canvas" height="1080" width="1960" style="outline: black 3px solid;"/>
        }
    }
}
