use yew::prelude::*;
use yew_router::prelude::*;

pub mod canvas;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/"]
    Home,
    #[to = "/connect4-computer"]
    Connect4Computer,
    #[to = "/connect4-human"]
    Connect4Human,
    #[to = "/toot-otto-computer"]
    TootOttoComputer,
    #[to = "/toot-otto-human"]
    TootOttoHuman,
}

pub struct App {
    clicked: bool,
    //onclick: Callback<ClickEvent>,
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
            //onclick: link.callback(|_| Msg::Click),
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
                    <Router<AppRoute>
                        render = Router::render(|switch: AppRoute| {

                            let header = match switch {
                                AppRoute::Home => html!{<h1>{"HomePage. Try /connect4-computer"}</h1>},
                                AppRoute::Connect4Computer => html!{<h1>{"Connect4Computer"}</h1>},
                                AppRoute::Connect4Human => html!{<h1>{"Connect4Human"}</h1>},
                                AppRoute::TootOttoComputer => html!{<h1>{"TootOttoComputer"}</h1>},
                                AppRoute::TootOttoHuman => html!{<h1>{"TootOttoHuman"}</h1>},
                            };
                            //let c = html!{<canvas id="canvas" height="475" width="640" style="outline: black 3px solid;"/>};
                            //if let AppRoute::Home != switch{
                            //    header
                            //}
                            let s = format!("{:?}", switch);
                            html!{
                                <div>
                                {s}
                                {header}
                                <p>{"hmmm"}</p>
                                </div>
                            }
        /*
                            html!{
                                {header}
                                {c}
                            }
                            */
                        })
                    />
                }
    }
}
