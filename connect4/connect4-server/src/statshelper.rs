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
        games_drawed: games_drawed(&all_games, username),
        games_ongoing: ongoing_games(&all_games, username),
    })
}

fn games_drawed(games: &Vec<GameData>, username: &str) -> isize {
    games
        .iter()
        .filter(|g| g.users.iter().any(|u| u == username))
        .filter(|g| g.board_state == BoardState::Draw)
        .collect::<Vec<&GameData>>()
        .len() as isize
}

fn games_won(games: &Vec<GameData>, username: &str) -> isize {
    games
        .iter()
        .filter_map(|g| match g.board_state {
            BoardState::Win(w) => Some((g, w-1)),
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
            g.users[*w as usize - 1] != username
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
    use connect4_lib::games;
    static P1: &str = "StatsMcGee";
    static P2: &str = "TestyMcTestface";

    fn mock_game_data(board_state: BoardState, users: Vec<&str>) -> GameData {
        GameData {
            roomcode: "".to_string(),
            board_state: board_state,
            users: users.iter().map(|s| s.to_string()).collect(),
            game: games::connect4(),
        }
    }

    fn mock_games_vec() -> Vec<GameData> {
        vec![
        mock_game_data(BoardState::Win(1), vec![P1, P2]),
        mock_game_data(BoardState::Win(1), vec![P1, P2]),
        mock_game_data(BoardState::Win(2), vec![P1, P2]),
        mock_game_data(BoardState::Ongoing, vec![P1,P2]),
        mock_game_data(BoardState::Draw, vec![P1, P2]),
        ]
    }

    #[test]
    fn lost_stats_test() {
        assert_eq!(1, games_lost(&mock_games_vec(), P1));
    }
    #[test]
    fn won_stats_test() {
        assert_eq!(2, games_won(&mock_games_vec(), P1));
    }
    #[test]
    fn ongoing_stats_test() {
        assert_eq!(1, ongoing_games(&mock_games_vec(), P1));
    }
    #[test]
    fn draw_stats_test() {
        assert_eq!(1, games_drawed(&mock_games_vec(), P1));
    }

    #[test]
    fn zero_stats_test() {
        let games = vec![];
        assert_eq!(0, games_lost(&games, P1));
        assert_eq!(0, games_won(&games, P1));
        assert_eq!(0, ongoing_games(&games, P1));
        assert_eq!(0, games_drawed(&games, P1));
    }

    #[test]
    #[ignore]
    fn db_endpoint_test() {
        //db.drop(None); // foo bar everything
        // TODO: this
    }
}
