use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use connect4_coms::{
    status,
    types::{GameData, GameDataResponse, PlayMove, Signin},
};
use connect4_lib::game::Chip;

use crate::log;

const SERVER_LOC: &'static str = "localhost:8000";
pub fn test_request() {
    async fn test() {
        request("GET", "getgame", None, None).await;
    }

    log(&format!("Spawning local for request"));
    spawn_local(test());
}

pub async fn getgame(id: &str) -> Option<GameData> {
    let js_json = request("GET", &format!("getgame/{}", id), None, None).await;
    match js_json.map(|x| x.into_serde::<GameDataResponse>()) {
        Ok(Ok(v)) => {
            if v.status == status::SUCCESS {
                v.game_data
            } else {
                None
            }
        }
        _ => None,
    }
}

pub async fn signin(usr: &str, passwd: &str) -> Option<String> {
    let js_json = request("GET", &format!("signin/{}/{}", usr, passwd), None, None).await;
    match js_json.map(|x| x.into_serde::<Signin>()) {
        Ok(Ok(v)) => {
            if v.status == status::SUCCESS {
                Some(v.tok)
            } else {
                None
            }
        }
        _ => None,
    }
}

pub async fn playmove(chip: &Chip) -> Option<isize> {
    let js_json = request("PUT", "playmove", None, None).await;
    match js_json.map(|x| x.into_serde::<PlayMove>()) {
        Ok(Ok(v)) => {
            if v.status == status::SUCCESS {
                Some(v.column)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn build_url(postfix: &str) -> String {
    format!("http://{}/api/{}", SERVER_LOC, postfix)
}

async fn request(
    verb: &str,
    path: &str,
    body: Option<&str>,
    tok: Option<&str>,
) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method(verb);
    opts.mode(RequestMode::Cors);

    log(&format!("Making request to: {}", build_url(path)));
    let request: Result<Request, JsValue> = Request::new_with_str_and_init(&build_url(path), &opts);
    let request = request?;

    if let Some(m_tok) = tok {
        request
            .headers()
            .set("Authorization", &format!("Bearer {}", m_tok))?;
    }
    opts.body(body.map(|x| JsValue::from_str(x)).as_ref());
    if body.is_some() {
        request.headers().set("Content-Type", "application/json")?;
    }

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    log(&format!("Got data: {:?}", resp));
    // Convert this other `Promise` into a rust `Future`.
    let res = JsFuture::from(resp.json()?).await?;
    log(&format!("It was Okay"));
    Ok(res)
}
