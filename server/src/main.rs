#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::{Stream};

use std::io::{self};
use std::fs::File;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/")]
fn root() -> io::Result<NamedFile> {
    NamedFile::open("/www/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("/www/").join(file)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![root, files]).launch();
}
