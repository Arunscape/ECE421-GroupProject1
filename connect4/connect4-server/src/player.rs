use crate::dbhelper::*;
use crate::jwthelper::*;
use bson::doc;
use connect4_coms::types::ClaimPayload;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
    password: String,
}

// given username and password, possibly sign in for JWT token
pub fn sign_in(username: &str, password: &str) -> Option<String> {
    let user_doc = object_to_doc(&User {
        username: username.to_string(),
        password: password.to_string(),
    })?;

    // can connect to DB
    let db = new_db(DATABASE_NAME)?;
    // username not in db, add then JWT
    if !exists_any_in(
        &db,
        USER_COLLECTION_NAME,
        doc! {"username": username.to_owned()},
    ) {

        // insert into db, return None if that fails
        if db.collection(USER_COLLECTION_NAME).insert_one(user_doc, None).is_err() {
            return None;
        }

        return Some(gen_jwt_token(
            ClaimPayload{username: username.to_string()},
            JWT_LIFETIME_SECONDS,
        ));
    }

    // They exist in the database
    if exists_any_in(&db, USER_COLLECTION_NAME, user_doc) {
        Some(gen_jwt_token(
            ClaimPayload{username: username.to_string()},
            JWT_LIFETIME_SECONDS,
        ))
    } else {
        // invalid password
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn db_sign_in_test() {
        let _token = sign_in("Alex", "Yeet").expect("Alex shouldn't be in the DB yet");

        let _token = sign_in("Alex", "Yeet").expect("Alex must sign in again");

        // this isnt Alex's password!!
        let token = sign_in("Alex", "Yote");
        assert!(token == None);
    }
}
