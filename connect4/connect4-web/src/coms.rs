use crate::storage::LocalStorage;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use connect4_coms::{
    status,
    types::{
        GameData, GameDataResponse, GameStats, GameStatsResponse, JoinPlayers, JoinPlayersResponse,
        PlayMove, Signin,
    },
};
use connect4_lib::game::{Chip, Game, PlayerType};

use crate::{console_log, log};

const SERVER_LOC: &'static str = env!("SERVER_URL");

pub async fn getgame(id: &str) -> Option<GameData> {
    let token = LocalStorage::get_token();
    let js_json = request::<i32>("GET", &format!("getgame/{}", id), None, token).await;
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

pub async fn create_game(game: Game) -> Option<GameData> {
    let token = LocalStorage::get_token();
    let js_json = request("PUT", &"creategame", Some(game), token).await;
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

pub async fn join_game(game_id: &str, plays: Vec<PlayerType>) -> Option<Vec<Option<isize>>> {
    let token = LocalStorage::get_token();
    let body = JoinPlayers { players: plays };
    let js_json = request("PUT", &format!("joingame/{}", game_id), Some(body), token).await;
    match js_json.map(|x| x.into_serde::<JoinPlayersResponse>()) {
        Ok(Ok(v)) => {
            if v.status == status::SUCCESS {
                Some(v.player_numbers)
            } else {
                None
            }
        }
        _ => None,
    }
}

pub async fn refresh() {
    let token = LocalStorage::get_token();
    if token.is_some() {
        let js_json = request::<i32>("POST", "refresh", None, token).await;
        match js_json.map(|x| x.into_serde::<Signin>()) {
            Ok(Ok(v)) => {
                if v.status == status::SUCCESS {
                    LocalStorage::set_token(&v.tok);
                } else {
                    LocalStorage::clear_token();
                }
            }
            _ => {
                LocalStorage::clear_token();
                while let Err(_) = crate::window().location().reload() {}
            }
        }
    }
}

pub fn sync_refresh() {
    async fn asyncr() {
        refresh().await;
        console_log!("Refreshed Token");
    }
    spawn_local(asyncr());
}

pub async fn signin(usr: &str, passwd: &str) -> Option<String> {
    let hashed_passwd = hash_password(passwd)?;

    let js_json = request::<i32>(
        "GET",
        &format!("signin/{}/{}", usr, &hashed_passwd),
        None,
        None,
    )
    .await;
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

fn hash_password(passwd: &str) -> Option<String> {
    // NOTE: this salt is not secret, for security, it is okay for anyone to know this
    let salt = b"DQ63&CUSv@s&@g1I&mC0MP4mTuoA0yU^V45ChiO6Z&$^$nS&TOy2381u*TBdJMEoewZp6jkn5E4lec8#8UblyKZ1yiyQRAupmvr3sumvjXbH1Iu!5*FOekAI6upIun1tPRO^z9HuV1mMm#mIP3MqcPDtW7j7OJOUQdvrwNoM$g8H0RV&Yb%h!wz&9Qz9e%uEqT^Q1BjcDhk&En^Zy08$FL8tqfJtYpxi*SHbnpkKEEo2nPLh2pItvjDo7mj2lY1v";
    let config = argon2::Config::default();
    match argon2::hash_encoded(passwd.as_bytes(), salt, &config) {
        Ok(h) => Some(String::from(js_sys::encode_uri_component(&h))),
        _ => None,
    }
}

pub async fn playmove(chip: Chip, game_id: String) -> Option<GameData> {
    let play = PlayMove {
        game_id,
        column: chip.get_x(),
        chip_descrip: chip.get_descrip(),
    };
    let token = LocalStorage::get_token();
    let js_json = request("PUT", "playmove", Some(play), token).await;
    match js_json.map(|x| x.into_serde::<GameDataResponse>()) {
        Ok(Ok(v)) => v.game_data,
        _ => None,
    }
}

pub async fn getgamespast() -> Vec<GameData> {
    getgames(true).await
}

pub async fn getgamespresent() -> Vec<GameData> {
    getgames(false).await
}

async fn getgames(past: bool) -> Vec<GameData> {
    let token = LocalStorage::get_token();
    let url = if past {
        "allgames/past"
    } else {
        "allgames/ongoing"
    };
    let js_json = request::<i32>("GET", url, None, token).await;
    match js_json.map(|x| x.into_serde::<Vec<GameData>>()) {
        Ok(Ok(v)) => v,
        _ => Vec::new(),
    }
}

pub async fn get_player_stats() -> Option<GameStats> {
    let token = LocalStorage::get_token();
    let js_json = request::<i32>("GET", "playerstats", None, token).await;
    match js_json.map(|x| x.into_serde::<GameStatsResponse>()) {
        Ok(Ok(v)) => v.game_stats,
        _ => None,
    }
}

fn build_url(postfix: &str) -> String {
    format!("https://{}/api/{}", SERVER_LOC, postfix)
}

async fn request<T: Serialize>(
    verb: &str,
    path: &str,
    body: Option<T>,
    tok: Option<String>,
) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method(verb);
    opts.mode(RequestMode::Cors);
    let has_body = body.is_some();
    if let Some(b) = body {
        let body = serde_json::to_string(&b).unwrap();
        opts.body(Some(&body.into()));
    }

    //log(&format!("Making request to: {}", build_url(path)));
    let request: Result<Request, JsValue> = Request::new_with_str_and_init(&build_url(path), &opts);
    let request = request?;

    if let Some(m_tok) = tok {
        request
            .headers()
            .set("Authorization", &format!("Bearer {}", m_tok))?;
    }
    if has_body {
        request.headers().set("Content-Type", "application/json")?;
    }

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    //log(&format!("Got data: {:?}", resp));
    // Convert this other `Promise` into a rust `Future`.
    let res = JsFuture::from(resp.json()?).await?;
    //log(&format!("It was Okay"));
    Ok(res)
}
