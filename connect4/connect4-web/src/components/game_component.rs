use yew::prelude::*;
use crate::game_object::GameObject;
use connect4_lib::{game::Game, game::Board, games, game};
use crate::canvas::Canvas;

pub struct GameComponent {
    link: ComponentLink<Self>,
    props: Self::Properties,
    game_object: GameObject,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub game_type: &'static str,
    pub active: bool,
    pub other_player: &'static str,
}

pub enum Msg {
    GoBack,
}

impl GameComponent {
    // I OWN THE GAME OBJECT AND DECIDE WHAT TO DO WITH THE PROPS I RECEIVE
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {

        let game_type = match props.game_type{
            constants::game::CONNECT4 => games::GameType::Connect4,
            constants::game::TOTO => games::GameType::Toto,
            _ => unreachable!(),
        };

        let game = match props.player_type{
            constants::player::AI_EASY => games::build_game(game_type, game::PlayerType::AI(game::ai::EASY_AI), game::PlayerType::Local),
            constants::player::AI_EASY2 => games::build_game(game_type, game::PlayerType::Local, game::PlayerType::AI(game::ai::EASY_AI)),
            constants::player::AI_MID => games::build_game(game_type, game::PlayerType::AI(game::ai::MID_AI), game::PlayerType::Local),
            constants::player::AI_MID2 => games::build_game(game_type, game::PlayerType::Local, game::PlayerType::AI(game::ai::MID_AI))
            constants::player::AI_HARD => games::build_game(game_type, game::PlayerType::AI(game::ai::HARD_AI), game::PlayerType::Local),
            constants::player::AI_HARD2  => games::build_game(game_type, game::PlayerType::Local, game::PlayerType::AI(game::ai::HARD_AI)),
            constants::player::LOCAL   =>  games::build_game(game_type, game::PlayerType::Local, game::PlayerType::Local,
            constants::player::REMOTE => games::build_game(game_type, game::PlayerType::Local, game::PlayerType::Remote),
        };

        let canvas_id = "todocreaterandomcanvasid"; // TODO
        let canvas = Canvas::new(canvas_id);
        
        let game_object = GameObject {
            game,
            canvas,

        }
        Self { props, link, game_object }
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
