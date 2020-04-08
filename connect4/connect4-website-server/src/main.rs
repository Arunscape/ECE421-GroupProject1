#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::NamedFile;
use rocket_contrib::serve::StaticFiles;
use std::io;
use std::path::PathBuf;

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
    let path = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("connect4-web");
    println!("{}", path.to_str().unwrap());
    //rocket::ignite().mount("/", StaticFiles::from(path))
    rocket::ignite().mount("/", routes![index, files])
}

fn main() {
    rocket().launch();
}
