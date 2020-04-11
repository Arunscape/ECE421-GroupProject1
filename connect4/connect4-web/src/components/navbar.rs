use super::super::AppRoute;
use yew::{prelude::*, virtual_dom::VNode, Properties};
use yew_router::prelude::*;

pub struct Navbar;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub enum Msg {}

impl Component for Navbar {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> VNode {
        html! {
        <nav class="w3-sidenav w3-red w3-collapse w3-top w3-large w3-padding" style="z-index:3;width:350px;font-weight:bold"
            id="mySidenav"><br/>
            <a href="javascript:void(0)" class="w3-padding-xlarge w3-hide-large w3-display-topleft w3-hover-white"
            style="width:100%">{"Close Menu"}</a>
            <div class="w3-container">
                <h3 class="w3-padding-64">
                    <b>{"Play"}
                    <br/>
                    {"Connect4 / TOOT-OTTO"}
                    </b>
                </h3>
            </div>
            <RouterAnchor<AppRoute> route=AppRoute::HowToConnect4 classes="w3-padding w3-hover-white">{"How to Play Connect4"}</RouterAnchor<AppRoute>>
            <RouterAnchor<AppRoute> route=AppRoute::Connect4Computer classes="w3-padding w3-hover-white">{"Play Connect4 With Computer"}</RouterAnchor<AppRoute>>
            <RouterAnchor<AppRoute> route=AppRoute::Connect4Human classes="w3-padding w3-hover-white">{"Play Connect4 with Another Human"}</RouterAnchor<AppRoute>>
                <br/>
            <RouterAnchor<AppRoute> route=AppRoute::HowToToot classes="w3-padding w3-hover-white">{"How to Play TOOT-OTTO"}</RouterAnchor<AppRoute>>
            <RouterAnchor<AppRoute> route=AppRoute::TootOttoComputer classes="w3-padding w3-hover-white">{"Play Toot-Otto With Computer"}</RouterAnchor<AppRoute>>
            <RouterAnchor<AppRoute> route=AppRoute::TootOttoHuman classes="w3-padding w3-hover-white">{"Play Toot-Otto With Another Human"}</RouterAnchor<AppRoute>>
                <br/>
            <RouterAnchor<AppRoute> route=AppRoute::ScoreBoard classes="w3-padding w3-hover-white">{"View Game History"}</RouterAnchor<AppRoute>>
            <RouterAnchor<AppRoute> route=AppRoute::Scores classes="w3-padding w3-hover-white">{"Score Board"}</RouterAnchor<AppRoute>>
        </nav>
            }
    }
}
