#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::NamedFile;
use rocket_contrib::serve::StaticFiles;
use std::{io, path::PathBuf};

mod exampleDB;

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

#[get("/")]
fn index() -> io::Result<NamedFile> {
    let path = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("connect4-web/index.html");
    NamedFile::open(path)
}

// allow html to reference any file with path /static under folder "static"
#[get("/<_file..>", rank = 10)] // use rank here to allow other api endpoint available as well
fn files(_file: PathBuf) -> io::Result<NamedFile> {
    let path = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("connect4-web/index.html");
    NamedFile::open(path)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/api",
            routes![signin, playmove, refresh, creategame, getgame],
        )
        .mount("/", routes![index, files])
}

fn main() {
	//exampleDB::add_chip();
    rocket().launch();
}
