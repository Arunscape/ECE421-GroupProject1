pub mod types {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Signin{
        pub status: String,
        pub tok: String,
    }
}
