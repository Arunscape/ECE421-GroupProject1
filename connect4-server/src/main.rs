#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::NamedFile;
use rocket_contrib::serve::StaticFiles;
use std::io;

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", StaticFiles::from("www"))
}

fn main() {
    rocket().launch();
}
