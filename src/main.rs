#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin, custom_attribute)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_derive;

extern crate base64;

extern crate multipart;
extern crate mime;
extern crate url;
extern crate raster;
extern crate rayon;
extern crate graceful;
extern crate tempfile;
extern crate rand;


use rocket::http::{ContentType, Status};
use rocket::response::Stream;
use rocket::response::status::Custom;
use rocket::{Data, Rocket};
use rocket_contrib::serve::StaticFiles;
use std::io::{self, Cursor, Read};

mod functions;
mod models;

use functions::*;
use std::fs::File;

use graceful::SignalGuard;
use std::thread::sleep;

//#[post("/imgtest/v1", format="json", data = "<data>")]
//// signature requires the request to have a `Content-Type`
//fn imgtestjson(cont_type: &ContentType, data: Data) -> Result<Stream<Cursor<Vec<u8>>>, Custom<String>> {
//    // this and the next check can be implemented as a request guard but it seems like just
//    // more boilerplate than necessary
////    if !cont_type.is_form_data() {
////        return Err(Custom(
////            Status::BadRequest,
////            "Content-Type not multipart/form-data".into(),
////        ));
////    }
////
////    let (_, boundary) = cont_type.params().find(|&(k, _)| k == "boundary").ok_or_else(
////        || Custom(
////            Status::BadRequest,
////            "`Content-Type: multipart/form-data` boundary param not provided".into(),
////        )
////    )?;
////
////    match process_upload(boundary, data) {
////        Ok(resp) => Ok(Stream::from(Cursor::new(resp))),
////        Err(err) => Err(Custom(Status::InternalServerError, err.to_string()))
////    }
//}

#[get("/")]
fn index() -> &'static str {

    // riros read readme.md -> html
//     Custom(Status::Ok, "<html><b>ok</b></html>")
    "readme md file"
}

#[get("/favicon.ico")]
fn favicon() -> io::Result<Stream<File>> {
    File::open("favicon.ico").map(|file| Stream::from(file))
}

#[post("/imgtest/v1", format = "multipart/form-data", data = "<data>")]
// signature requires the request to have a `Content-Type`
fn imgtestform(cont_type: &ContentType, data: Data) -> Result<Stream<Cursor<Vec<u8>>>, Custom<String>> {
    // this and the next check can be implemented as a request guard but it seems like just
    // more boilerplate than necessary
//    if !cont_type.is_form_data() {
//        return Err(Custom(
//            Status::BadRequest,
//            "Content-Type not multipart/form-data".into(),
//        ));
//    }

    let (_, boundary) = cont_type.params().find(|&(k, _)| k == "boundary").ok_or_else(
        || Custom(
            Status::BadRequest,
            "`Content-Type: multipart/form-data` boundary param not provided".into(),
        )
    )?;

    match process_upload(boundary, data) {
        Ok(resp) => Ok(Stream::from(Cursor::new(resp))),
        Err(err) => Err(Custom(Status::InternalServerError, err.to_string()))
    }
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", routes![
        imgtestform,
        index,
        favicon,
        ])
        .mount("/static", StaticFiles::from("static"))
}


fn main() {
//    let signal_guard = SignalGuard::new();
//    signal_guard.at_exit(move |sig| {
//        println!("Signal {} received.", sig);
//        while rayon::current_num_threads() > 0 {
//            println!("wait thread work, sleep 1 sec ...");
//            sleep(std::time::Duration::new(1, 0));
//        }
//    });

    println!("Start server...");
    rocket().launch();
    println!("Shutting down server...");

}