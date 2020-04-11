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
            .set_item(&"auth", value);
    }
}
