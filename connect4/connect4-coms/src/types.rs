use connect4_lib::game::ChipDescrip;
use connect4_lib::game::{BoardState, Game};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Signin {
    pub status: String,
    pub tok: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Refresh {
    pub status: String,
    pub new_tok: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayMove {
    pub status: String,
    pub game_id: String,
    pub column: isize,
    pub chip_descrip: ChipDescrip,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameData {
    pub roomcode: String,
    pub board_state: BoardState,
    pub users: Vec<String>,

    #[serde(flatten)]
    pub game: Game,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameDataResponse {
    pub status: String,
    pub game_data: GameData,
}
