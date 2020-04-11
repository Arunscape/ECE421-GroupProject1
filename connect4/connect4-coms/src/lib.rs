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
        pub roomcode: String,
        pub board_state: BoardState,
        pub users: Vec<String>,

        #[serde(flatten)]
        pub game: Game,
    }

}

