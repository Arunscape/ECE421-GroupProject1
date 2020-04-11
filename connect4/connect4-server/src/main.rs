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

mod jwtHelper;
mod player;
mod dbhelper;
mod gamehelper;

/// /signin: takes username and password, returns JWT
#[get("/signin")]
fn signin() -> &'static str {
    "Signin"
}

/// /playmove: takes in description of move, gameid, and JWT, returns new gamestate
#[get("/playmove")]
fn playmove() -> &'static str {
    "playmove"
}

/// /refresh: takes in JWT returns new JWT
#[get("/refresh")]
fn refresh() -> &'static str {
    "refresh"
}

/// /creategame: takes in description of game, and JWT, returns gameid
#[get("/creategame")]
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

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    let path = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("connect4-web")
        .join(file);
    println!("{:?}", path);
    NamedFile::open(path).ok()
}

#[get("/connect4-computer")]
fn connect4human() -> Option<NamedFile> {
    let path = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("connect4-web/index.html");
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
        .mount("/", StaticFiles::from(path))
        .register(catchers![not_found])
        .mount("/", routes![files, connect4human])
}

fn main() {
    //exampleDB::add_chip();
    rocket().launch();
}
