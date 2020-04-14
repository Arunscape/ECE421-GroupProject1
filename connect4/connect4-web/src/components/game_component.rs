use crate::canvas::Canvas;
use crate::constants::{game, player};
use crate::game_object::GameObject;
use connect4_lib::{
    ai::{EASY_AI, HARD_AI, MID_AI},
    game::Board,
    game::Game,
    game::PlayerType::{Local, Remote, AI},
    games,
};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use yew::prelude::*;

pub struct GameComponent {
    link: ComponentLink<Self>,
    props: Props,
    game_object: Option<GameObject>,
    canvas_id: String,
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
        let canvas_id: String = String::from("canvas")
            + &thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .collect::<String>()
                .to_ascii_lowercase();

        Self {
            props,
            link,
            canvas_id,
            game_object: None,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        let canvas = Canvas::new(self.canvas_id.clone());

        let game_type = match self.props.game_type.as_str() {
            game::CONNECT4 => games::GameType::Connect4,
            game::TOTO => games::GameType::Toto,
            _ => unreachable!(),
        };

        let game = match self.props.other_player.as_str() {
            player::AI_EASY => games::build_game(game_type, AI(EASY_AI), Local),
            player::AI_EASY2 => games::build_game(game_type, Local, AI(EASY_AI)),
            player::AI_MID => games::build_game(game_type, AI(MID_AI), Local),
            player::AI_MID2 => games::build_game(game_type, Local, AI(MID_AI)),
            player::AI_HARD => games::build_game(game_type, AI(HARD_AI), Local),
            player::AI_HARD2 => games::build_game(game_type, Local, AI(HARD_AI)),
            player::LOCAL => games::build_game(game_type, Local, Local),
            player::REMOTE => games::build_game(game_type, Local, Remote),
            _ => todo!(),
        };

        let game = GameObject::new(canvas, game);
        self.game_object = Some(game);
        true
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <canvas id={&self.canvas_id} class="h-full w-full" height="1080" width="1960" style="outline: black 3px solid;"/>
        }
    }
}
