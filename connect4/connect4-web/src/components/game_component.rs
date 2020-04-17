use crate::canvas::Canvas;
use crate::constants::{game, player};
use crate::game_object::GameObject;
use connect4_lib::{
    ai::{EASY_AI, HARD_AI, MID_AI},
    game::PlayerType::{Local, Remote, AI},
    games,
};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use wasm_bindgen::JsCast;
use yew::prelude::*;

pub struct GameComponent {
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
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let canvas_id: String = String::from("canvas")
            + &thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .collect::<String>()
                .to_ascii_lowercase();

        Self {
            props,
            canvas_id,
            game_object: None,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        let canvas = Canvas::new(self.canvas_id.clone());

        let game_type = match self.props.game_type.as_str() {
            game::CONNECT4 => games::GameType::Connect4,
            game::TOTO => games::GameType::Toto,
            _ => games::GameType::Connect4, // TODO: better default
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
            _ => games::build_game(game_type, Remote, Remote), // server will just replace this anyways
        };

        let game = GameObject::new(canvas, self.props.active, game, self.props.gameid.clone());
        self.game_object = Some(game);
        self.update_size();
        true
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        self.update_size();
        html! {
           <canvas id={&self.canvas_id} height="1080" width="1960"
                      class="h-full w-full" style="outline: black 3px solid;"/>
        }
    }
}
impl GameComponent {
    fn update_size(&self) {
        let canvas = crate::document().get_element_by_id(&self.canvas_id);
        if let Some(canvas) = canvas {
            let parent = canvas.parent_element().unwrap();
            let canvas: web_sys::HtmlCanvasElement = canvas
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .map_err(|_| ())
                .unwrap();
            let sharpness = 3;
            let bounds = parent.get_bounding_client_rect();
            canvas.set_height(sharpness * (bounds.height() as u32));
            canvas.set_width(sharpness * (bounds.width() as u32));
        }
    }
}
