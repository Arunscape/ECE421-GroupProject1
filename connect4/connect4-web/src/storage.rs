use web_sys::window;
use jsonwebtoken::{dangerous_unsafe_decode};
use connect4_coms::types::{Claims, ClaimPayload};

pub struct LocalStorage {}

impl LocalStorage {
    pub fn get_token() -> Option<String> {
        window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item(&"auth")
            .unwrap()
    }
    pub fn set_token(value: &str) {
        window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .set_item(&"auth", value);
    }

    pub fn get_username() -> Option<String> {
        Self::get_token()
            .and_then(|x| claims_from_jwt_token(x))
            .and_then(|x| match x.data {
              ClaimPayload::username(s) => Some(s),
              _ => None,
            })
    }
}


pub fn claims_from_jwt_token(token: String) -> Option<Claims> {
    match dangerous_unsafe_decode::<Claims>(&token) {
        Ok(c) => Some(c.claims),
        Err(_) => None,
    }
}
