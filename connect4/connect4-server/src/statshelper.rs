use crate::dbhelper::*;
use bson::doc;
use connect4_lib::game::BoardState;
use connect4_coms::types::{GameData, GameStats};

pub fn get_stats(username: &str) -> Option<GameStats> {
    let db = new_db(DATABASE_NAME).expect("No mongo, is it running?");
    let all_games: Vec<GameData> = docs_to_objects(
    query_collection_for_docs(
        &db, GAME_COLLECTION_NAME,
        doc!{}
    ));

    Some(
    GameStats {
        player: username.to_string(),
        games_won: 0,
        games_lost: 0,
        games_ongoing: 0,
    })
}

fn games_won(games: &Vec<GameData>, username: &str) -> isize {
    games.iter()
    .filter(|g| g.users.iter().any(|u| u == username))
    .collect::<Vec<&GameData>>()
    .len() as isize
}

fn games_lost(games: &Vec<GameData>, username: &str) -> isize {
    games.iter()
    .filter(|g| g.users.iter().any(|u| u == username))
    .filter(|g|
        g.users
        .iter()
        .enumerate()
        .filter(|(_i, item)| item.as_str() == username)
        .map(|(i, _item)| i)
        .collect::<Vec<usize>>()
        .remove(0) == 0
    )
    .collect::<Vec<&GameData>>()
    .len() as isize
}

fn ongoing_games(games: &Vec<GameData>, username: &str) -> isize {
    games.iter()
    .filter(|g| g.users.iter().any(|u| u == username))
    .filter(|g| g.board_state == BoardState::Ongoing)
    .collect::<Vec<&GameData>>()
    .len() as isize
}

#[cfg(test)]
mod test {
    use super::*;
    //use connect4_lib::games;

    #[test]
    #[ignore]
    fn db_insert_game_test() {
    }
}
