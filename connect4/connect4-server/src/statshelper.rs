use crate::dbhelper::*;
use bson::doc;
use connect4_coms::types::{GameData, GameStats};
use connect4_lib::game::BoardState;

pub fn get_stats(username: &str) -> Option<GameStats> {
    let db = new_db(DATABASE_NAME).expect("No mongo, is it running?");
    let all_games: Vec<GameData> = docs_to_objects(query_collection_for_docs(
        &db,
        GAME_COLLECTION_NAME,
        doc! {},
    ));

    Some(GameStats {
        player: username.to_string(),
        games_won: games_won(&all_games, username),
        games_lost: games_lost(&all_games, username),
        games_ongoing: ongoing_games(&all_games, username),
    })
}

fn games_won(games: &Vec<GameData>, username: &str) -> isize {
    games
        .iter()
        .filter_map(|g| match g.board_state {
            BoardState::Win(w) => Some((g, w)),
            _ => None,
        })
        .filter(|(g, w)| {
            g.users
                .iter()
                .enumerate()
                .any(|(i, u)| u == username && *w == i as isize)
        })
        .collect::<Vec<_>>()
        .len() as isize
}

fn games_lost(games: &Vec<GameData>, username: &str) -> isize {
    games
        .iter()
        .filter_map(|g| match g.board_state {
            BoardState::Win(w) => Some((g, w)),
            _ => None,
        })
        .filter(|(g, w)| {
            g.users
                .iter()
                .enumerate()
                .all(|(i, u)| u != username && *w != i as isize)
        })
        .collect::<Vec<_>>()
        .len() as isize
}

fn ongoing_games(games: &Vec<GameData>, username: &str) -> isize {
    games
        .iter()
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
    fn db_insert_game_test() {}
}
