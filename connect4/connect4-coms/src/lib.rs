pub mod types {
    use serde::{Deserialize, Serialize};
    use connect4_lib::game::{BoardState, Game};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Signin{
        pub status: String,
        pub tok: String,
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

