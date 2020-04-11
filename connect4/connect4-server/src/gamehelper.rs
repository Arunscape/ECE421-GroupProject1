use crate::jwtHelper::*;
use crate::dbhelper::*;
use bson::doc;
use serde::{Deserialize, Serialize};

use connect4_lib::{
    game, game::Board, game::BoardState, game::ChipDescrip, game::Game, games, io, GameIO,
};


#[derive(Debug, Serialize, Deserialize)]
struct GameData {
    game: game::Game,
    board_state: game::BoardState,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn db_server_test() {
    }
}
