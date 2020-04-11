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
        pub game_id: String,
        pub column: isize,
        pub chip_descrip: ChipDescrip,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GameData {
        roomcode: String,
        board_state: BoardState,
        users: Vec<String>,

        #[serde(flatten)]
        game: Game,
    }
}
