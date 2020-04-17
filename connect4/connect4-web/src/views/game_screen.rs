use yew::prelude::*;

use crate::{components::{GameComponent, MenuButtonLight}};

use crate::components::router::query;

use crate::{console_log, log};

pub struct GameScreen {
}

impl Component for GameScreen {
    type Message = ();
    type Properties = ();
    // I OWN THE GAME COMPONENT
    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let gt = query("game").unwrap_or(String::from("connect4"));
        let op = query("player").unwrap_or(String::from("local"));
        let id = get_game_code();
        console_log!("Creating game component for room: {}", id);
        html! {
          <div class="w-screen h-screen flex md:flex-col flex-col-reverse md:justify-end md:justify-start">
            <div class="w-full flex justify-center md:justify-end md:py-4">
              <MenuButtonLight dest="/" text="Home"/>
            </div>
            <div class="h-full w-full">
              <GameComponent game_type=gt other_player=op gameid=id active=true/>
            </div>
          </div>
        }
    }
}

fn get_game_code() -> String {
    let url = crate::window().location().href().unwrap();
    url.split('?')
        .next()
        .and_then(|x| x.split('/').skip(4).next())
        .map(|x| String::from(x))
        .unwrap_or(String::from("offline"))
}
