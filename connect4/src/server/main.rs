#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::NamedFile;
use rocket_contrib::serve::StaticFiles;
use std::io;

/// /signin: takes username and password, returns JWT
#[get("/signin")]
fn index() -> &'static str {
    "Hello, world!"
}

/// /refresh: takes in JWT returns new JWT

/// /creategame: takes in description of game, and JWT, returns gameid

/// /playmove: takes in description of move, gameid, and JWT, returns new gamestate

/// /getgame: takes in gameid, JWT, and returns gamestate
#[get("/refresh")]
fn index() -> &'static str {
    "Hello, world!"
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", StaticFiles::from("www"))
}

fn main() {
    rocket().launch();
}
