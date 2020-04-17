use yew::prelude::*;

pub enum ConnectIcon {
    Settings,
    Stats,
    ToggleOn,
    ToggleOff,
}

pub fn html(i: ConnectIcon) -> Html {
    match i {
        ConnectIcon::Settings => html! {<span class="material-icons">{"settings"}</span>},
        ConnectIcon::Stats => html! {<span class="material-icons">{"bar_chart"}</span>},
        ConnectIcon::ToggleOff => html! {<span class="material-icons">{"check_box"}</span>},
        ConnectIcon::ToggleOn => html! {<span class="material-icons">{"check_box_outline_blank"}</span>},
    }
}
