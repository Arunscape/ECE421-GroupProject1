use crate::dbhelper::*;
use bson::doc;
use connect4_lib::game;
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

fn games_won(db: &mongodb::Database, username: &str) -> isize {
    0
}

fn games_lost(db: &mongodb::Database, username: &str) -> isize {
    0
}

fn ongoing_games(db: &mongodb::Database, username: &str) -> isize {
    0
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
