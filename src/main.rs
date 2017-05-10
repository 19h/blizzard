#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::response::{Stream};

use std::io::{self};
use std::fs::File;
use std::path::{Path, PathBuf};

fn get_file_stream(file: PathBuf) -> io::Result<Stream<File>> {
    File::open(Path::new("static/").join(file)).map(|file| Stream::from(file))
}

#[get("/")]
fn root() -> io::Result<Stream<File>> {
    get_file_stream(PathBuf::from("index.html"))
}

#[get("/<file..>")]
fn files(file: PathBuf) -> io::Result<Stream<File>> {
    get_file_stream(file)
}

fn main() {
    rocket::ignite().mount("/", routes![root, files]).launch();
}