// from https://github.com/Keats/jsonwebtoken/blob/master/examples/validation.rs

//use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time};

#[derive(Debug, Serialize, Deserialize)]
pub enum ClaimPayload {
    // dummy example payloads
    username(String),
    number(i32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    data: ClaimPayload, // extra data fields
    exp: usize,
}

fn since_epoch_seconds() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

fn jwt_token_from_claims(my_claims: Claims) -> String {
    let key = b"TODO:probablyshouldhidethis";
    match encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(key),
    ) {
        Ok(t) => t,
        Err(_) => panic!(),
        // in practice you would return the error
    }
}

pub fn claims_from_jwt_token(token: String) -> Option<Claims> {
    let key = b"TODO:probablyshouldhidethis";
    let validation = Validation {
        //sub: Some("b@b.com".to_string()), // more validation here
        ..Validation::default()
    };
    match decode::<Claims>(&token, &DecodingKey::from_secret(key), &validation) {
        Ok(c) => Some(c.claims),
        Err(_) => None,
    }
}

pub fn gen_jwt_token(payload: ClaimPayload, expires_in_seconds: u64) -> String {
    let my_claims = Claims {
        data: payload,
        exp: (since_epoch_seconds() + expires_in_seconds) as usize,
    };
    let token = jwt_token_from_claims(my_claims);
    token
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_jwt_test() {
        let token = gen_jwt_token(ClaimPayload::username("cats".to_owned()), 2);

        thread::sleep(time::Duration::from_millis(1 * 1000));

        let claims = claims_from_jwt_token(token).expect("still valid");
    }

    #[test]
    fn invalid_jwt_test() {
        let token = gen_jwt_token(ClaimPayload::username("cats".to_owned()), 1);

        thread::sleep(time::Duration::from_millis(2 * 1000));

        match claims_from_jwt_token(token) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn jwt_payload_test() {
        let token = gen_jwt_token(ClaimPayload::username("cats".to_owned()), 2);

        thread::sleep(time::Duration::from_millis(1 * 1000));

        let claims = claims_from_jwt_token(token).expect("Still valid");
        if let ClaimPayload::username(u) = claims.data {
            assert!(u == "cats".to_string());
        }
    }
}
