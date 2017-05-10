#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::response::{Stream};

use std::io::{self};
use std::fs::File;
use std::path::{Path, PathBuf};

#[get("/<file..>")]
fn files(file: PathBuf) -> io::Result<Stream<File>> {
    File::open(Path::new("static/").join(file)).map(|file| Stream::from(file))
}

fn main() {
    rocket::ignite().mount("/", routes![files]).launch();
}