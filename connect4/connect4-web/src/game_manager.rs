use wasm_bindgen_futures::spawn_local;

use crate::{window, constants};
use crate::coms;
use connect4_lib::{game, game::Game, games};

pub fn create_game(game_type: String) -> Game {
    let game_type = match game_type {
        s if s == constants::game::CONNECT4 => games::GameType::Connect4,
        s if s == constants::game::TOTO => games::GameType::Toto,
        _ => unreachable!(),
    };
    games::build_game(game_type, game::PlayerType::Local, game::PlayerType::Remote)
}

pub async fn initiate_game(game: Game) -> String {
    let game = coms::create_game(game).await;
    match game {
        Some(game_data) => {
            join_game(game_data.roomcode).await
        }
        None => {
            crate::alert("failed to create game");
            String::new()
        },
    }
}

pub async fn join_game(roomcode: String) -> String{
    let url = window().location().href().unwrap();
    let querystring = url
        .split('?')
        .skip(1)
        .next()
        .expect("failed to get querystring");
    let spots = coms::join_game(&roomcode).await;
    match spots {
        Some(s) => {
            if !s.iter().any(|x| x.is_none()) {
                format!(
                    "game/{}?{}",
                    roomcode,
                    querystring.to_string()
                )
            } else {
                crate::alert("Room is full!");
                String::new()
            }
        }
        _ => { crate::alert("Could not find game"); String::new() },
    }
}

pub fn create_game_and_go(game: Game) {
    async fn asyncer(game: Game) {
        let loc = initiate_game(game).await;
        window().location().set_href(&loc).unwrap();
    }
    spawn_local(asyncer(game));
}

pub fn join_game_and_go(roomcode: String) {
    async fn asyncer(roomcode: String) {
        let loc = join_game(roomcode).await;
        window().location().set_href(&loc).unwrap();
    }
    spawn_local(asyncer(roomcode));
}
