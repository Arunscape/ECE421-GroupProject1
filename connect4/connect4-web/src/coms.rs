use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::log;

const SERVER_LOC: &'static str = "localhost:8000";
pub fn test_request() {
    async fn with_errors() -> Result<(), JsValue> {
        log("Started Test Request call");
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let url = format!("http://{}/api/getgame", SERVER_LOC);

        log(&format!("Making request to: {}", url));
        let request: Result<Request, JsValue> = Request::new_with_str_and_init(&url, &opts);
        let request = request?;

        let tok = "None";
        request.headers().set("Authorization", &format!("Bearer {}", tok))?;

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

        // `resp_value` is a `Response` object.
        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().unwrap();

        // Convert this other `Promise` into a rust `Future`.
        let res = JsFuture::from(resp.text()?).await?;

        log(&format!("{:?}", res));
        log("Finishing Test Request call");
        Ok(())
    }
    log("Calling Test request");
    async fn test() {
      let val = with_errors().await;
      log(&format!("{:?}", val));
    }
    spawn_local(test());
}
