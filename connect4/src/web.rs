use yew::{html, Callback, ClickEvent, Component, ComponentLink, Html, ShouldRender};

pub struct App {
    clicked: bool,
    onclick: Callback<ClickEvent>,
}

pub enum Msg {
    Click,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            clicked: false,
            onclick: link.callback(|_| Msg::Click),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = true;
                true // Indicate that the Component should re-render
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.clicked {
            "Clicked!"
        } else {
            "Click me!"
        };

        html! {
        //            <div>
        //                <button onclick=&self.onclick>{ button_text }</button>
                        <canvas id="gameboard" height="480" width="640" style="outline: black 3px solid;"/>
        //            </div>
                }
    }
}
