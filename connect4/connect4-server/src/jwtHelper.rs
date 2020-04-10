// from https://github.com/Keats/jsonwebtoken/blob/master/examples/validation.rs

//use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time};



#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // extra data fields
    exp: usize,
}

fn since_epoch_seconds() -> u64 {
	match SystemTime::now().duration_since(UNIX_EPOCH) {
	    Ok(n) => n.as_secs(),
	    Err(_) => panic!("SystemTime before UNIX EPOCH!"),
	}
}


fn get_jwt_token(my_claims: Claims) -> String {
    let key = b"TODO:probablyshouldhidethis";
    match encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(key)) {
            Ok(t) => t,
            Err(_) => panic!(),
        // in practice you would return the error
    }
}


fn is_valid_jwt_token(token: String)
    -> Result<Claims, ()> {
    let key = b"TODO:probablyshouldhidethis";
    let validation = Validation {
        //sub: Some("b@b.com".to_string()), // more validation here
         ..Validation::default()
    };
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(key),
        &validation)
    {
        Ok(c) => Ok(c.claims),
        Err(_) => Err(()),
    }
}

fn example_usage() {
    let my_claims = Claims {
        sub: "asdf".to_owned(),
         exp: (since_epoch_seconds() + 6) as usize,
    };

    let token = get_jwt_token(my_claims);

    thread::sleep(time::Duration::from_millis( 5 * 1000));

    if let Ok(claim) = is_valid_jwt_token(token) {
        println!("{:?}", claim);
    } else {
        println!("not a valid thing");
    }
}
