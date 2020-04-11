#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::{ContentType, Status};
use rocket::response::status::NotFound;
use rocket::response::{NamedFile, Redirect};
use rocket::Request;
use rocket::Response;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::fs::File;
use std::{io, path::PathBuf};

mod dbhelper;
mod gamehelper;
mod jwtHelper;
mod player;

/// /signin: takes username and password, returns JWT
#[get("/signin/<u>/<p>", rank = 9)]
fn signin(u: String, p: String) -> Option<String> {
    println!("Signin called [{}, {}]", u, p);
    player::sign_in(u.as_str(), p.as_str())
}

/// /playmove: takes in description of move, gameid, and JWT, returns new gamestate
#[put("/playmove")]
fn playmove() -> &'static str {
    "playmove"
}

/// /refresh: takes in JWT returns new JWT
#[post("/refresh")]
fn refresh() -> &'static str {
    "refresh"
}

/// /creategame: takes in description of game, and JWT, returns gameid
#[put("/creategame")]
fn creategame() -> &'static str {
    "creategame"
}

/// /getgame: takes in gameid, JWT, and returns gamestate
#[get("/getgame")]
fn getgame() -> &'static str {
    "getgame"
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
        .mount("/", StaticFiles::from(path))
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
