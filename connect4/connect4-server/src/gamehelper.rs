use crate::dbhelper::*;
use bson::doc;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use connect4_lib::game;

static ROOM_CODE_LEN: usize = 3;

use connect4_coms::types::{GameData, JoinPlayers};

fn valid_play(game_data: &GameData, username: &str, col: isize, color: game::ChipDescrip) -> bool {
    let valid_chip = game_data
        .game
        .current_player()
        .chip_options
        .iter()
        .fold(false, |valid_chip, chip| valid_chip || *chip == color);
    let valid_col = !game_data.game.invalid_column(col);

    let valid_turn_num = game_data
        .users
        .iter()
        .enumerate()
        .filter(|(_i, u)| username == u.to_string())
        .map(|(i, _)| i)
        .any(|player_num| {
            (game_data.game.get_turn() as usize % game_data.game.get_player_count()) == player_num
        });

    valid_turn_num && valid_chip && valid_col
}

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
        roomcode = gen_roomcode();
    }
}

// given a connect4-lib style game, insert it into the DB
// TODO: adding placeholder AI's in the users
pub fn insert_new_game(game_maker: &str, game: game::Game) -> Option<GameData> {
    let mut new_game = GameData {
        roomcode: gen_valid_roomcode().to_owned(),
        board_state: game::BoardState::Ongoing,
        users: vec![],
        game: game,
    };

    let db = new_db(DATABASE_NAME).expect("No mongo, is it running?");

    let game_doc = object_to_doc(&new_game)?;

    if db
        .collection(GAME_COLLECTION_NAME)
        .insert_one(game_doc, None)
        .is_err()
    {
        return None;
    }

    adjust_local_perspective(&mut new_game, game_maker);
    Some(new_game)
}

// TODO: refactor to take in a Comms PlayMove object
pub fn update_game_with_play(
    roomcode: &str,
    username: &str,
    col: isize,
    color: game::ChipDescrip,
) -> Option<GameData> {
    let db = new_db(DATABASE_NAME).expect("No mongo, is it running?");
    if let Some(mut game_data) = get_game_data(username, roomcode) {
        if !valid_play(&game_data, username, col, color) {
            return None;
        }
        // make the play
        game_data.board_state = game_data.game.play(col, color);

        // update the DB
        if db
            .collection(GAME_COLLECTION_NAME)
            .replace_one(
                doc! {"roomcode": roomcode.to_string()},
                object_to_doc(&game_data)?,
                None,
            )
            .is_err()
        {
            return None;
        }

        // return updated data
        adjust_local_perspective(&mut game_data, username);
        Some(game_data)
    } else {
        None
    }
}

pub fn get_game_data(username: &str, roomcode: &str) -> Option<GameData> {
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

    adjust_local_perspective(&mut game_data, username);
    Some(game_data)
}

// with respoect to the username, adjust game_data's player types
// to local and remote accordingly, skip the AI's
fn adjust_local_perspective(game_data: &mut GameData, username: &str) {
    if game_data.users.len() == 0 {
        // nothing to do
        return;
    }

    for i in 0..game_data.users.len() {
        game_data.game.players[i].player_type = {
            if let game::PlayerType::AI(asdf) = game_data.game.players[i].player_type {
                game::PlayerType::AI(asdf)
            } else if game_data.users[i] == username {
                game::PlayerType::Local
            } else {
                game::PlayerType::Remote
            }
        };
    }
}

// add the player to game_data's users as username
// return the player number in the array 0-indexed
fn insert_player(
    game_data: &mut GameData,
    username: &str,
    player: game::PlayerType,
) -> Option<isize> {
    let players_in_game = game_data.users.len();
    let player_count = game_data.game.get_player_count();
    if players_in_game == player_count {
        // Game is full
        None
    } else {
        // assert game_data has this player number as an AI
        // if this new player is an AI

        if let game::PlayerType::AI(_) = player {
            if let game::PlayerType::AI(_) = game_data.game.get_player(players_in_game).player_type
            {
                game_data.users.push(username.to_string());
                return Some(players_in_game as isize);
            } else {
                return None;
            }
        } else {
            if let game::PlayerType::AI(_) = game_data.game.get_player(players_in_game).player_type
            {
                return None;
            } else {
                game_data.users.push(username.to_string());
                return Some(players_in_game as isize);
            }

        }

    }
}

fn insert_players(
    game_data: &mut GameData,
    username: &str,
    players: Vec<game::PlayerType>,
) -> Vec<Option<isize>> {
    players
        .iter()
        .map(|p| insert_player(game_data, username, p.clone()))
        .collect()
}

pub fn join_players(roomcode: &str, username: &str, joining: JoinPlayers) -> Vec<Option<isize>> {
    let mut game_data = get_game_data(username, roomcode).expect("GameData, is mogno running?");
    let res = insert_players(&mut game_data, username, joining.players);

    // write new users to the database
    // todo: return vec of none's if the write fails
    let db = new_db(DATABASE_NAME).expect("No mongo, is it running?");
    if db
        .collection(GAME_COLLECTION_NAME)
        .replace_one(
            doc! {"roomcode": roomcode.to_string()},
            object_to_doc(&game_data).expect("should go todoc??"),
            None,
        )
        .is_err()
    {
        vec![None; res.len()]
    } else {
        res
    }
}

pub fn all_ongoing_games(username: &str) -> Vec<GameData> {
    let db = new_db(DATABASE_NAME).expect("no mongo, is it running?");
    docs_to_objects(query_collection_for_docs(
        &db,
        GAME_COLLECTION_NAME,
        doc! {"$and" : [{"users": {"$elemMatch": {"$eq": username.to_string() }}},{"board_state": "Ongoing"}  ]},
    ))
}
pub fn all_not_ongoing_games(username: &str) -> Vec<GameData> {
    let db = new_db(DATABASE_NAME).expect("no mongo, is it running?");
    docs_to_objects(query_collection_for_docs(
        &db,
        GAME_COLLECTION_NAME,
        doc! {"$and" : [{"users": {"$elemMatch": {"$eq": username.to_string()}}},{"board_state": {"$ne": "Ongoing"}}  ]},
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use connect4_lib::ai::HARD_AI;
    use connect4_lib::games;

    fn mock_ai_game_data() -> GameData {
        GameData {
            roomcode: "".to_string(),
            board_state: game::BoardState::Ongoing,
            users: vec![],
            game: games::connect4_ai(),
        }
    }

    #[test]
    fn join_ai_invalid_test() {
        let mut game_data = mock_ai_game_data();
        let res = insert_players(
            &mut game_data,
            "asdf",
            vec![game::PlayerType::Local, game::PlayerType::AI(HARD_AI)],
        );
        assert_eq!(res, vec![None, Some(0)]);
    }

    #[test]
    fn join_ai_valid_test() {
        let mut game_data = mock_ai_game_data();
        let res = insert_players(
            &mut game_data,
            "asdf",
            vec![game::PlayerType::AI(HARD_AI), game::PlayerType::Local],
        );

        // player 1 must be an AI
        assert_eq!(res, vec![Some(0), Some(1)]);
    }

    #[test]
    #[ignore]
    fn db_invalid_play_test() {
        let game: game::Game = games::connect4_3player();
        let game_data = insert_new_game("Alex", game).expect("GameData");
        let roomcode = game_data.roomcode;
        // should panic, Alex hasnt joined game yet
        assert!(update_game_with_play(&roomcode, "Alex", 1, games::YELLOW_CHIP).is_none());
    }

    #[test]
    #[ignore]
    fn db_multi_client_join_play_test() {
        let game: game::Game = games::connect4_3player();

        // /api/newgame
        let game_data = insert_new_game("Alex", game).expect("GameData");
        let roomcode = game_data.roomcode;

        // /api/joinplayers/<roomcode>
        let result = join_players(
            &roomcode,
            "Alex",
            JoinPlayers {
                players: vec![game::PlayerType::Local, game::PlayerType::Local],
            },
        );
        assert!(result == vec![Some(0 as isize), Some(1 as isize)]);

        // /api/playmove/<roomcode>
        update_game_with_play(&roomcode, "Alex", 1, games::YELLOW_CHIP);
        // arun joins from a second client
        // /api/joinplayers/<roomcode>
        let result = join_players(
            &roomcode,
            "Arun",
            JoinPlayers {
                players: vec![game::PlayerType::Local],
            },
        );
        assert!(result == vec![Some(2 as isize)]);
    }

    #[test]
    #[ignore]
    fn db_add_players_to_game_test() {
        let user1 = "Alex";
        let user2 = "Arun";
        let game: game::Game = games::connect4_3player();
        let game_data = insert_new_game(user1, game).expect("GameData");
        let roomcode = game_data.roomcode;
        update_game_with_play(&roomcode, user1, 1, games::YELLOW_CHIP);

        // side effect: this adds user2 to the game
        get_game_data(user2, &roomcode);
        let user2_sees = get_game_data(user2, &roomcode).expect("GameData should exist");
        let user1_sees = get_game_data(user1, &roomcode).expect("GameData should exist");
        assert!(user2_sees.users == user1_sees.users);
    }

    #[test]
    fn invalid_plays_test() {
        let p1 = "Alex";
        let p2 = "Arun";
        let game: game::Game = games::connect4();
        let mut players = game.players.clone();
        let red = players.remove(0).chip_options.remove(0);
        let yellow = players.remove(0).chip_options.remove(0);
        let game_data = GameData {
            users: vec![p1.to_string(), p2.to_string()],
            roomcode: "".to_string(),
            board_state: game::BoardState::Ongoing,
            game: game,
        };
        // username, column, color

        // not p2's turn
        assert!(!valid_play(&game_data, p2, 1, yellow));
        // not valid colum
        assert!(!valid_play(&game_data, p1, 10, red));
        // not a p1's color
        assert!(!valid_play(&game_data, p1, 1, yellow));

        // p1 valid play
        assert!(valid_play(&game_data, p1, 1, red));
    }

    #[test]
    fn play_yourself_valid_test() {
        let p1 = "Alex";
        let p2 = "Alex";
        let game: game::Game = games::connect4();
        let mut players = game.players.clone();
        let red = players.remove(0).chip_options.remove(0);
        let yellow = players.remove(0).chip_options.remove(0);
        let game_data = GameData {
            users: vec![p1.to_string(), p2.to_string()],
            roomcode: "".to_string(),
            board_state: game::BoardState::Ongoing,
            game: game,
        };
        // username, column, color

        // multi users have the same name
        assert!(valid_play(&game_data, p1, 1, red));
        assert!(valid_play(&game_data, p2, 1, red));
        // wrong color test
        assert!(!valid_play(&game_data, p2, 1, yellow));
    }

    #[test]
    #[ignore]
    fn db_crush_arun_test() {
        let game: game::Game = games::connect4();
        let players = game.players.clone();

        // /api/newgame
        let game_data = insert_new_game("Alex", game).expect("GameData");
        let roomcode = game_data.roomcode;

        // /api/joinplayers/<roomcode>
        let _result = join_players(
            &roomcode,
            "Alex",
            JoinPlayers {
                players: vec![game::PlayerType::Local],
            },
        );
        let _result = join_players(
            &roomcode,
            "Arun",
            JoinPlayers {
                players: vec![game::PlayerType::Local],
            },
        );

        // /api/playmove/<roomcode>
        for i in 0..3 {
            update_game_with_play(&roomcode, "Alex", i, players[0].clone().chip_options[0]);
            update_game_with_play(&roomcode, "Arun", i, players[1].clone().chip_options[0]);
        }
        //play winning move
        update_game_with_play(&roomcode, "Alex", 3, players[0].clone().chip_options[0]);
    }
}
