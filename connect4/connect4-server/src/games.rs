use crate::jwtHelper::*;
use crate::dbhelper::*;
use bson::doc;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct Game {
    data: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn db_server_test() {
    }
}
