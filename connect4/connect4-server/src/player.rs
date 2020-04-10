use mongodb::{options::ClientOptions, Client};
//use bson::{doc, bson, to_bson};
use bson::ordered::OrderedDocument;
use bson::*;
use connect4_lib::{
    game, game::Board, game::BoardState, game::ChipDescrip, game::Game, games, io, GameIO,
};
use crate::jwtHelper::*;

static databaseName: &str = "Connect4DB";
static jwtLifetimeSeconds: u64 = 5;

// return mongodb database object associated with databaseName
// database
fn get_db(db_name: &str) -> Result<mongodb::Database, mongodb::error::Error> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")?;
    let client = Client::with_options(client_options)?;
    let db = client.database(db_name);
    Ok(db)
}

// bool if the document is in collection_name collection for the
// databaseName database
fn in_collection(collection_name: &str, doc: bson::Document) -> bool {
    if let Ok(db) = get_db(databaseName) {
        match db.collection(collection_name).find(doc, None) {
            Ok(mut cursor) => match cursor.next() {
                Some(_) => true,
                None => false,
            },
            Err(_) => {
                return false;
            }
        }
    } else {
        // some error getting DB
        false
    }
}

<<<<<<< HEAD
=======

// given username and password, possibly sign in for JWT token
fn sign_in(username: &str, _password: &str) -> Option<String> {

    if in_collection(
        "players",
        doc!{"username": username.to_string()})
    {
        return None;
    }
    Some(
        gen_jwt_token(
            ClaimPayload::username(username.to_string()),
            jwtLifetimeSeconds)
    )
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn db_in_collection_test() {
        assert!(in_collection("players", doc! {}));
        assert!(!in_collection("players", doc! {"YEET":"NOTIN"}));
    }

    #[test]
    #[ignore]
    fn db_sign_in_test() {
        match sign_in("Alex", "Yeet") {
            Some(_) => assert!(true),
            None => assert!(false),
        }
    }
}
