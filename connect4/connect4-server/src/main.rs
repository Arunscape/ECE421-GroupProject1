#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::response::{content, NamedFile};
use rocket::Outcome;
use rocket::Request;
use rocket_contrib::json::Json;
use std::path::PathBuf;
//use rocket::response::status::NotFound;
//use rocket::Response;
//use rocket_contrib::serve::StaticFiles;
//use rocket_contrib::templates::Template;
//use std::fs::File;

mod dbhelper;
mod gamehelper;
mod jwthelper;
mod player;
mod statshelper;

use connect4_coms::types::{ClaimPayload, PlayMove};
use connect4_coms::types::{
    GameDataResponse, GameStatsResponse, JoinPlayers, JoinPlayersResponse, Signin,
};
use jwthelper::{claims_from_jwt_token, gen_jwt_token};

// if a handler has this type in its params,
// then the handler will have a valid claim payload
// OR the request will fail
struct JwtPayloadWrapper {
    claim_payload: ClaimPayload,
}

// extract jwt from header, then extract claim data
// fail with ()???
impl<'a, 'r> FromRequest<'a, 'r> for JwtPayloadWrapper {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        //println!("{:?}", request.headers());
        let token: String = request
            .headers()
            .get("Authorization")
            .next()
            .expect("no authorization in header") // TODO: dont panic
            .split(" ")
            .skip(1) // skip the word bearer
            .next()
            .expect("no jwt token in header")
            .to_string();
        //println!("Parsed JWT token: {:?}", token);
        match claims_from_jwt_token(token) {
            Some(claim) => Outcome::Success(JwtPayloadWrapper {
                claim_payload: claim.data,
            }),
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

impl JwtPayloadWrapper {
    fn get_username(&self) -> Option<&str> {
        if &self.claim_payload.username != "" {
            Some(&self.claim_payload.username)
        } else {
            None
        }
    }
}

#[get("/signin/<u>/<p>")]
fn signin(u: String, p: String) -> content::Json<String> {
    println!("Signin called [{}, {}]", u, p);
    let data = match player::sign_in(u.as_str(), p.as_str()) {
        Some(s) => Signin {
            tok: s,
            status: String::from("success"),
        },
        None => Signin {
            tok: String::from(""),
            status: String::from("success"),
        },
    };
    content::Json(serde_json::to_string(&data).unwrap())
}

#[get("/playerstats")]
fn getstats(wrapper: JwtPayloadWrapper) -> content::Json<String> {
    let data = match wrapper.get_username() {
        Some(u) => GameStatsResponse {
            status: String::from("success"),
            game_stats: statshelper::get_stats(u),
        },
        None => GameStatsResponse {
            status: String::from("failed"),
            game_stats: None,
        },
    };

    content::Json(serde_json::to_string(&data).unwrap())
}

#[put("/playmove", data = "<move_data>")]
fn playmove(wrapper: JwtPayloadWrapper, move_data: Json<PlayMove>) -> content::Json<String> {
    // get data according to jwt username extraction success
    let mut data = match wrapper.get_username() {
        Some(u) => GameDataResponse {
            status: String::from("success"),
            game_data: gamehelper::update_game_with_play(
                &move_data.game_id,
                u,
                move_data.column,
                move_data.chip_descrip,
            ),
        },
        None => GameDataResponse {
            status: String::from("failed"),
            game_data: None,
        },
    };

    // if play update failed change error message
    if !data.game_data.is_some() {
        data.status = String::from("Invalid move?");
    }

    content::Json(serde_json::to_string(&data).unwrap())
}

#[post("/refresh")]
fn refresh(wrapper: JwtPayloadWrapper) -> content::Json<String> {
    // get data according to jwt username extraction success
    let data = match wrapper.get_username() {
        Some(u) => Signin {
            status: String::from("success"),
            tok: gen_jwt_token(
                ClaimPayload { username: u.to_string() },
                dbhelper::JWT_LIFETIME_SECONDS,
            ),
        },
        None => Signin {
            status: String::from("failed"),
            tok: String::from(""),
        },
    };

    content::Json(serde_json::to_string(&data).unwrap())
}

#[get("/allgames/ongoing")]
fn allongoing(wrapper: JwtPayloadWrapper) -> content::Json<String> {
    // get data according to jwt username extraction success
    let data = match wrapper.get_username() {
        Some(u) => gamehelper::all_ongoing_games(u),
        None => vec![],
    };

    content::Json(serde_json::to_string(&data).unwrap())
}

#[get("/allgames/past")]
fn allpast(wrapper: JwtPayloadWrapper) -> content::Json<String> {
    // get data according to jwt username extraction success
    let data = match wrapper.get_username() {
        Some(u) => gamehelper::all_not_ongoing_games(u),
        None => vec![],
    };

    content::Json(serde_json::to_string(&data).unwrap())
}

#[put("/creategame", format = "application/json", data = "<new_game>")]
fn creategame(
    wrapper: JwtPayloadWrapper,
    new_game: Json<connect4_lib::game::Game>,
) -> content::Json<String> {
    let mut data = match wrapper.get_username() {
        Some(u) => GameDataResponse {
            status: String::from("success"),
            game_data: gamehelper::insert_new_game(u, new_game.into_inner()),
        },
        None => GameDataResponse {
            status: String::from("No Username in JWT"),
            game_data: None,
        },
    };

    // if get_game_data failed change error message
    if !data.game_data.is_some() {
        data.status = String::from("could not find game");
    }

    content::Json(serde_json::to_string(&data).unwrap())
}

#[put("/joingame/<id>", data = "<new_players>")]
fn joingame(
    wrapper: JwtPayloadWrapper,
    id: String,
    new_players: Json<JoinPlayers>,
) -> content::Json<String> {
    let data = match wrapper.get_username() {
        Some(u) => JoinPlayersResponse {
            status: String::from("success"),
            player_numbers: gamehelper::join_players(&id, u, new_players.into_inner()),
        },
        None => JoinPlayersResponse {
            status: String::from("failed to add any players"),
            player_numbers: vec![None; new_players.players.len()],
        },
    };

    content::Json(serde_json::to_string(&data).unwrap())
}

#[get("/getgame/<id>")]
fn getgame(id: String, wrapper: JwtPayloadWrapper) -> content::Json<String> {
    let mut data = match wrapper.get_username() {
        Some(u) => GameDataResponse {
            status: String::from("success"),
            game_data: gamehelper::get_game_data(u, id.as_str()),
        },
        None => GameDataResponse {
            status: String::from("No Username in JWT"),
            game_data: None,
        },
    };

    // if get_game_data failed change error message
    if !data.game_data.is_some() {
        data.status = String::from("could not find game");
    }

    content::Json(serde_json::to_string(&data).unwrap())
}

#[catch(404)]
fn not_found<'a>(_req: &Request) -> Option<NamedFile> {
    let path = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("connect4-web/index.html");
    NamedFile::open(path).ok()
}

#[get("/<file..>", rank = 11)]
fn files(file: PathBuf) -> Option<NamedFile> {
    let path = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("connect4-web/pkg")
        .join(file);
    println!("{:?}", path);
    NamedFile::open(path).ok()
}

fn allow_default_cors() -> rocket_cors::Cors {
    match rocket_cors::CorsOptions::default().to_cors() {
        Err(e) => panic!("Rocket Cors Error: {:?}", e),
        Ok(cors) => cors,
    }
}

fn rocket() -> rocket::Rocket {
    let _path = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("connect4-web/index.html");
    rocket::ignite()
        .mount(
            "/api",
            routes![
                getstats, allpast, allongoing, joingame, signin, playmove, refresh, creategame,
                getgame
            ],
        )
        .mount("/pkg", routes![files])
        .attach(allow_default_cors())
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
