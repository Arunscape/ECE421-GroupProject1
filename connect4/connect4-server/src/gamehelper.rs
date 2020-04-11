use crate::dbhelper::*;
use crate::jwtHelper::*;
use crate::player::*;
use bson::doc;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use connect4_lib::{
    game, game::Board, game::BoardState, game::ChipDescrip, game::Game, games, io, GameIO,
};

static ROOM_CODE_LEN: usize = 3;

// from https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
fn gen_roomcode() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(ROOM_CODE_LEN)
        .collect();
    //rand_string.to_ascii_uppercase()
    rand_string
}

fn gen_valid_roomcode() -> String {
    let mut roomcode = gen_roomcode();
    let db = new_db(DATABASE_NAME).expect("No mongo, is it running?");
    loop {
        if !exists_any_in(
            &db,
            GAME_COLLECTION_NAME,
            doc! {"roomcode": roomcode.to_owned()},
        ) {
            return roomcode;
        }
    }
    return "".to_string();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameData {
    roomcode: String,
    board_state: game::BoardState,
    users: Vec<String>,

    #[serde(flatten)]
    game: game::Game,
}

// given a connect4-lib style game, insert it into the DB
// TODO: adding placeholder AI's in the users
pub fn insert_new_game(game_maker: &str, game: game::Game) -> String {
    let mut new_game = GameData {
        roomcode: gen_valid_roomcode().to_owned(),
        board_state: game::BoardState::Ongoing, // is this default?
        users: vec![game_maker.to_string()],    //TODO: add players...
        game: game,
    };

    let db = new_db(DATABASE_NAME).expect("No mongo, is it running?");

    let game_doc = object_to_doc(&new_game).expect("GameData should go into a doc??");

    db.collection(GAME_COLLECTION_NAME)
        .insert_one(game_doc, None); // TODO: error handle

    new_game.roomcode.to_owned()
}

pub fn update_game_with_play(
    roomcode: String,
    username: &str,
    col: isize,
    color: game::ChipDescrip,
) -> Option<GameData> {
    let db = new_db(DATABASE_NAME).expect("No mongo, is it running?");
    if let Some(mut game_data) = get_game_by_roomcode(username, roomcode.as_str()) {
        if !valid_play(&game_data, username, col, color) {
            return None;
        }
        // make the play
        game_data.board_state = game_data.game.play(col, color);

        // update the DB
        db.collection(GAME_COLLECTION_NAME).replace_one(
            doc! {"roomcode": roomcode.to_owned()},
            object_to_doc(&game_data).expect("should go todoc??"),
            None,
        );

        // return updated data
        Some(game_data)
    } else {
        None
    }
}

fn valid_play(game_data: &GameData, username: &str, col: isize, color: game::ChipDescrip) -> bool {
    true
}

pub fn get_game_by_roomcode(username: &str, roomcode: &str) -> Option<GameData> {
    let db = new_db(DATABASE_NAME).expect("No mongo, is it running?");

    let game_docs = query_collection_for_docs(
        &db,
        GAME_COLLECTION_NAME,
        doc! {"roomcode": roomcode.to_owned()},
    );

    // sould only be one game doc
    if game_docs.len() == 0 {
        return None;
    }

    let mut game_data: GameData = docs_to_objects::<GameData>(game_docs).remove(0);

    possibly_add_username_to_game(&mut game_data, username);
    Some(game_data)
}

fn whats_my_player_number(game_data: &GameData, username: &str) -> Option<isize> {
    let res: Vec<usize> = game_data
        .users
        .iter()
        .enumerate()
        .filter(|(i, item)| item.as_str() == username)
        .map(|(i, item)| i)
        .collect();

    if res.len() == 0 {
        None
    } else {
        Some(res[0] as isize)
    }
}

// side effect: user is added to the game if they are not already
fn possibly_add_username_to_game(game_data: &mut GameData, username: &str) {
    match whats_my_player_number(game_data, username) {
        Some(num) => { /*nothing to do here*/ }
        None => game_data.users.push(username.to_string()),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn db_game_test() {
        let game: game::Game = games::connect4_3player();
        let roomcode = insert_new_game("Alex", game);
        update_game_with_play(roomcode, "Alex", 1, games::YELLOW_CHIP);
    }
}
