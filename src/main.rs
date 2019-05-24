#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin, custom_attribute)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_derive;

extern crate base64;

#[cfg(test)]
mod tests;

//extern crate comrak;
extern crate graceful;
extern crate mime;
extern crate multipart;
extern crate rand;
extern crate raster;
extern crate rayon;
//extern crate tempfile;
extern crate url;

use rocket::http::{ContentType, Status};
use rocket::response::status::Custom;
use rocket::response::Stream;
use rocket::{Data, Request, Response, Rocket};
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

use comrak::{markdown_to_html, ComrakOptions};

use rocket::response::content;
use std::fs;

#[get("/ping")]
fn pong() -> content::Plain<String> {
    content::Plain("pong".to_string())
}

#[get("/")]
pub fn index() -> content::Html<String> {
    content::Html(format!(
        "{}{}{}",
        "<html>",
        markdown_to_html(
            fs::read_to_string("readme.md").unwrap().as_str(),
            &ComrakOptions::default(),
        ),
        "</html>"
    ))

    // riros read readme.md -> html
    //     Custom(Status::Ok, "<html><b>ok</b></html>")
}

#[get("/favicon.ico")]
fn favicon() -> io::Result<Stream<File>> {
    File::open("favicon.ico").map(|file| Stream::from(file))
}

#[post("/imgtest/v1", format = "multipart/form-data", data = "<data>")]
// signature requires the request to have a `Content-Type`
fn imgtestform(
    cont_type: &ContentType,
    data: Data,
) -> Result<Stream<Cursor<Vec<u8>>>, Custom<String>> {
    // this and the next check can be implemented as a request guard but it seems like just
    // more boilerplate than necessary
    //    if !cont_type.is_form_data() {
    //        return Err(Custom(
    //            Status::BadRequest,
    //            "Content-Type not multipart/form-data".into(),
    //        ));
    //    }

    let (_, boundary) = cont_type
        .params()
        .find(|&(k, _)| k == "boundary")
        .ok_or_else(|| {
            Custom(
                Status::BadRequest,
                "`Content-Type: multipart/form-data` boundary param not provided".into(),
            )
        })?;

    match process_upload(boundary, data) {
        Ok(resp) => Ok(Stream::from(Cursor::new(resp))),
        Err(err) => Err(Custom(Status::InternalServerError, err.to_string())),
    }
}

#[catch(404)]
fn not_found(request: &Request) -> content::Html<String> {
    let html = match request.format() {
        Some(ref mt) if !mt.is_json() && !mt.is_plain() => {
            format!("<p>'{}' requests are not supported.</p>", mt)
        }
        _ => format!(
            "<p>Sorry, '{}' is an invalid path! Try \
             /hello/&lt;name&gt;/&lt;age&gt; instead.</p>",
            request.uri()
        ),
    };

    content::Html(html)
}

pub fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", routes![imgtestform, index, favicon, pong,])
        .mount("/static", StaticFiles::from("static"))
        .register(catchers![not_found])
}

pub fn main() {
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
