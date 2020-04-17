use crate::dbhelper::*;
use crate::jwthelper::*;
use bson::doc;
use connect4_coms::types::ClaimPayload;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
    password: String,
    random_salt: String,
}

// given username and password, possibly sign in for JWT token
pub fn sign_in(username: &str, password: &str) -> Option<String> {
    // can connect to DB
    let db = new_db(DATABASE_NAME)?;
    let query = query_collection_for_docs(&db, USER_COLLECTION_NAME, doc! {"username": username});
    let client_hashed = match js_sys::decode_uri_component(password) {
        Ok(h) => Some(String::from(h)),
        _ => None,
    }?;
    let config = argon2::Config::default();
    // username not in db, add then JWT
    if query.len() == 0 {
        let random_salt = thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(64)
            .collect::<String>();

        let hashed_password = match argon2::hash_encoded(
            client_hashed.as_bytes(),
            &random_salt.as_bytes(),
            &config,
        ) {
            Ok(h) => Some(String::from(h)),
            _ => None,
        }?;

        let user_doc = object_to_doc(&User {
            username: username.to_string(),
            password: hashed_password,
            random_salt,
        })?;
        // insert into db, return None if that fails
        if db
            .collection(USER_COLLECTION_NAME)
            .insert_one(user_doc, None)
            .is_err()
        {
            return None;
        }

        return Some(gen_jwt_token(
            ClaimPayload {
                username: username.to_string(),
            },
            JWT_LIFETIME_SECONDS,
        ));
    }

    if query.len() > 1 {
        panic!("Username should be UNIQUE in database, but that's not the case here");
    }

    let user: User = bson_to_object(query[0])?;

    let hash = argon2::hash_encoded(
        client_hashed.as_bytes(),
        user.random_salt.as_bytes(),
        &config,
    )
    .unwrap();
    let matches = argon2::verify_encoded(&hash, user.password.as_bytes()).unwrap();

    // They exist in the database
    if matches {
        Some(gen_jwt_token(
            ClaimPayload {
                username: username.to_string(),
            },
            JWT_LIFETIME_SECONDS,
        ))
    } else {
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
