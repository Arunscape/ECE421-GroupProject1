use yew::prelude::*;
use crate::game_object::GameObject;
use connect4_lib::{game::Game, game::Board, games, game::PlayerType::{AI, Local, Remote}, ai::{EASY_AI, MID_AI, HARD_AI}};
use crate::canvas::Canvas;
use crate::constants::{game, player};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub struct GameComponent {
    link: ComponentLink<Self>,
    props: Props,
    game_object: GameObject,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub game_type: String,
    pub active: bool,
    pub other_player: String,
    pub gameid: String,
}

impl Component for GameComponent {
    type Message = ();
    type Properties = Props;
    // I OWN THE GAME OBJECT AND DECIDE WHAT TO DO WITH THE PROPS I RECEIVE
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {

        let game_type = match props.game_type.as_str() {
            game::CONNECT4 => games::GameType::Connect4,
            game::TOTO => games::GameType::Toto,
            _ => unreachable!(),
        };

        let game = match props.other_player.as_str() {
            player::AI_EASY => games::build_game(game_type, AI(EASY_AI), Local),
            player::AI_EASY2 => games::build_game(game_type, Local, AI(EASY_AI)),
            player::AI_MID => games::build_game(game_type, AI(MID_AI), Local),
            player::AI_MID2 => games::build_game(game_type, Local, AI(MID_AI)),
            player::AI_HARD => games::build_game(game_type, AI(HARD_AI), Local),
            player::AI_HARD2  => games::build_game(game_type, Local, AI(HARD_AI)),
            player::LOCAL   =>  games::build_game(game_type, Local, Local),
            player::REMOTE => games::build_game(game_type, Local, Remote),
            _ => todo!(),
        };

        let canvas_id: String = String::from("canvas") + &thread_rng()
          .sample_iter(&Alphanumeric)
          .take(10)
          .collect::<String>().to_ascii_lowercase();

        let canvas = Canvas::new(canvas_id);
        Self { props, link, game_object: GameObject::new(canvas, game) }
    }
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <canvas classes="h-full w-full" id="canvas" height="1080" width="1960" style="outline: black 3px solid;"/>
        }
    }
}
