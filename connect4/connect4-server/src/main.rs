#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::NamedFile;
use rocket_contrib::serve::StaticFiles;
use std::io;

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

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![signin, playmove, refresh, creategame, getgame])
}

fn main() {
	//exampleDB::db_info();
    rocket().launch();
}
