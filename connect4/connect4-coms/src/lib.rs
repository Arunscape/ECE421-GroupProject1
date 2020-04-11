pub mod types {
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
        pub status: String,
        pub game_id: String,
        pub column: isize,
        pub chip_descrip: ChipDescrip,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GameData {
        pub status: String,
        roomcode: String,
        board_state: BoardState,
        users: Vec<String>,

        #[serde(flatten)]
        game: Game,
    }
}

pub mod status {
    pub const SUCCESS: &'static str = "success";
    pub const INCORRECT_PASSWD: &'static str = "incorrect password";
}
