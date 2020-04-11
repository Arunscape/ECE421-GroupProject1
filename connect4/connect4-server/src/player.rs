use crate::jwtHelper::*;
use crate::dbhelper::*;
use bson::doc;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
    password: String,
}

// given username and password, possibly sign in for JWT token
pub fn sign_in(username: &str, password: &str) -> Option<String> {

    // TODO: how do i indent this?
    let user_doc = object_to_doc( &User{
            username: username.to_string(),
            password: password.to_string(), })
        .expect("Should be able to Doc Users?");

    // can connect to DB
    if let Some(db) = new_db(DATABASE_NAME) {
        // not in db, add then JWT
        if !exists_any_in(&db, USER_COLLECTION_NAME,
            doc!{"username": username.to_owned()}) {

            // TODO: error handle insertion
            db.collection(USER_COLLECTION_NAME)
            .insert_one(user_doc, None);

		    return Some(gen_jwt_token(
		        ClaimPayload::username(username.to_string()),
		        JWT_LIFETIME_SECONDS,
		    ))
        }

        // They exist in the database
        if exists_any_in(&db, USER_COLLECTION_NAME, user_doc) {
		    return Some(gen_jwt_token(
		        ClaimPayload::username(username.to_string()),
		        JWT_LIFETIME_SECONDS,
		    ))
        }

    }
    None

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn db_sign_in_test() {
        let token = sign_in("Alex", "Yeet")
            .expect("Alex shouldn't be in the DB yet");

        // TODO: verify that Alex
        let token = sign_in("Alex", "Yeet")
            .expect("Alex must sign in again");

        // this isnt Alex's password!!
        let token = sign_in("Alex" , "Yote");
        assert!(token == None);
    }
}
