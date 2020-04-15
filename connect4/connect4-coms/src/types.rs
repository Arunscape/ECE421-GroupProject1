use connect4_lib::game::ChipDescrip;
use connect4_lib::game::{BoardState, Game};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Signin {
    pub status: String,
    pub tok: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayMove {
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
    pub game_data: Option<GameData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClaimPayload {
    username(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub data: ClaimPayload, // extra data fields
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct GameStats {
    pub player: String,
    pub games_won: isize,
    pub games_lost: isize,
    pub games_drawed: isize,
    pub games_ongoing: isize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameStatsResponse {
    pub status: String,
    pub game_stats: Option<GameStats>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinPlayers {
    pub players: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinPlayersResponse {
    pub status: String,
    pub player_numbers: Vec<Option<isize>>,
}
