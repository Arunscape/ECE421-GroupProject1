// from https://github.com/Keats/jsonwebtoken/blob/master/examples/validation.rs

pub static JWT_KEY: &str = env!("JWT_KEY");

//use jsonwebtoken::errors::ErrorKind;
use connect4_coms::types::{ClaimPayload, Claims};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};

fn since_epoch_seconds() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

fn jwt_token_from_claims(my_claims: Claims) -> String {
    let key = JWT_KEY.as_bytes();
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
    let key = JWT_KEY.as_bytes();
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
    use std::{thread, time};

    fn get_token() -> String {
        gen_jwt_token(
            ClaimPayload {
                username: "cats".to_owned(),
            },
            2,
        )
    }

    #[test]
    fn valid_jwt_test() {
        let token = get_token();

        thread::sleep(time::Duration::from_millis(1 * 1000));

        let _claims = claims_from_jwt_token(token).expect("still valid");
    }

    #[test]
    fn invalid_jwt_test() {
        let token = get_token();

        thread::sleep(time::Duration::from_millis(3 * 1000));

        match claims_from_jwt_token(token) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn jwt_payload_test() {
        let token = get_token();
        thread::sleep(time::Duration::from_millis(1 * 1000));

        let claims = claims_from_jwt_token(token).expect("Still valid");
        assert!(claims.data.username == "cats".to_string());
    }
}
