use connect4_coms::types::Claims;
use jsonwebtoken::dangerous_unsafe_decode;
use web_sys::window;

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
            .set_item(&"auth", value)
            .expect("failed to set token");
    }
    pub fn clear_token() {
        window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .remove_item(&"auth")
            .expect("failed to clear token");
    }

    pub fn get_username() -> Option<String> {
        Self::get_token()
            .and_then(|x| claims_from_jwt_token(x))
            .map(|x| x.data.username)
    }

    pub fn set_colorblind_setting(val: bool) {
        window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .set_item(&"colorblind", &format!("{}", val))
            .expect("failed to set colourblind setting");
    }
    pub fn get_colorblind_setting() -> bool {
        window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item(&"colorblind")
            .unwrap()
            .unwrap_or(String::new())
            == "true"
    }
}

pub fn claims_from_jwt_token(token: String) -> Option<Claims> {
    match dangerous_unsafe_decode::<Claims>(&token) {
        Ok(c) => Some(c.claims),
        Err(_) => None,
    }
}
