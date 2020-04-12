pub mod types {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Signin{
        pub status: String,
        pub tok: String,
    }


    #[derive(Debug, Serialize, Deserialize)]
    pub enum ClaimPayload {
        // dummy example payloads
        username(String),
        number(i32),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Claims {
        pub data: ClaimPayload, // extra data fields
        pub exp: usize,
    }
}
