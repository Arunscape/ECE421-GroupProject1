pub mod types {
    use connect4_lib::game::ChipDescrip;
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

        #[serde(flatten)]
        pub chip_descrip: ChipDescrip,
    }
}
