#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::http::{ContentType, Status};
use rocket::response::status::NotFound;
use rocket::response::{content, NamedFile, Redirect};
use rocket::Request;
use rocket::Response;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::fs::File;
use std::{io, path::PathBuf};

use connect4_coms::types::{GameDataResponse, Refresh, Signin};

mod dbhelper;
mod gamehelper;
mod jwtHelper;
mod player;

use connect4_coms::types::{ClaimPayload, Claims, PlayMove};
use connect4_lib::game::ChipDescrip;
use connect4_lib::game::Game;
use connect4_lib::games::connect4_3player; // TODO: remove
use jwtHelper::{claims_from_jwt_token, gen_jwt_token}; //TODO: remove

use rocket::request::{self, FromRequest};
use rocket::Outcome;

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
        println!("{:?}", request.headers());
        let token: String = request
            .headers()
            .get("authorization")
            .next()
            .expect("no authorization in header")
            .split(" ")
            .skip(1) // skip the word bearer
            .next()
            .expect("no jwt token in header")
            .to_string();
        println!("Parsed JWT token: {:?}", token);
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
        if let ClaimPayload::username(u) = &self.claim_payload {
            Some(u)
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
        Some(u) => Refresh {
            status: String::from("success"),
            new_tok: gen_jwt_token(
                ClaimPayload::username(u.to_string()),
                dbhelper::JWT_LIFETIME_SECONDS,
            ),
        },
        None => Refresh {
            status: String::from("failed"),
            new_tok: String::from(""),
        },
    };

    content::Json(serde_json::to_string(&data).unwrap())
}

#[put("/creategame", data = "<new_game>")]
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
fn not_found<'a>(req: &Request) -> Option<NamedFile> {
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

fn rocket() -> rocket::Rocket {
    let path = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("connect4-web/index.html");
    rocket::ignite()
        .mount(
            "/api",
            routes![signin, playmove, refresh, creategame, getgame],
        )
        .mount("/pkg", routes![files])
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
