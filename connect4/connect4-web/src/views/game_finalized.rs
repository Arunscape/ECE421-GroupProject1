use crate::components::router::query;
use crate::constants;
use crate::game_manager;
use yew::prelude::*;

pub struct GameFinalized;

impl Component for GameFinalized {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn mounted(&mut self) -> ShouldRender {
        let mut player2 = query("player").unwrap_or(String::from(constants::player::LOCAL));
        let game_type = query("game").unwrap_or(String::from(constants::game::CONNECT4));

        if crate::storage::LocalStorage::get_username().is_none() {
            let loc = format!("/game/offline?player={}&game={}", player2, game_type);
            crate::window().location().set_href(&loc).unwrap();
            return false;
        }

        let mut player1 = String::from(constants::player::LOCAL);
        if player2 == constants::player::AI_HARD
            || player2 == constants::player::AI_MID
            || player2 == constants::player::AI_EASY
        {
            std::mem::swap(&mut player1, &mut player2);
        }

        let game = game_manager::create_game(
            game_type,
            constants::player::string_to_enum(&player1),
            constants::player::string_to_enum(&player2),
        );

        game_manager::create_game_and_go(game);

        false
    }

    fn view(&self) -> Html {
        html! {
            <p> { "Loading..." } </p>
        }
    }
}
