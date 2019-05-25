#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin, custom_attribute)]

#[macro_use]
extern crate rocket;
extern crate base64;
extern crate graceful;
extern crate mime;
extern crate multipart;
extern crate num_traits;
extern crate rand;
extern crate raster;
extern crate rayon;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_derive;
extern crate sysinfo;
extern crate url;

#[cfg(test)]
mod tests;

use rocket::http::{ContentType, Status};
use rocket::response::status::Custom;
use rocket::response::Responder;
use rocket::response::Stream;
use rocket::{Data, Request, Response, Rocket};
use rocket_contrib::serve::StaticFiles;
use std::io::{self, Cursor, Read};

mod functions;
mod models;

use functions::*;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use std::fs::File;

use comrak::{markdown_to_html, ComrakOptions};
use graceful::SignalGuard;
use rocket::response::content;
use std::fs;

#[post("/imgtest/v1", format = "json", data = "<message>")]
// signature requires the request to have a `Content-Type`
fn imgtestjson(
    //    cont_type: &ContentType,
    message: Json<models::Images>,
) -> Option<Json<&'static str>> {
    dbg!(&message);
    match process_json(message.into_inner()) {
        Ok(_) => Some(Json(r##"{"status":"ok"}"##)),
        Err(_) => Some(Json(r##"{"status":"error"}"##)),
    }
}

#[post("/imgtest/v1", format = "multipart/form-data", data = "<data>")]
// signature requires the request to have a `Content-Type`
fn imgtestform(
    cont_type: &ContentType,
    data: Data,
) -> Result<Stream<Cursor<Vec<u8>>>, Custom<String>> {
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

#[get("/ping")]
fn pong() -> content::Plain<String> {
    content::Plain("pong".to_string())
}

#[get("/")]
pub fn index() -> content::Html<String> {
    content::Html(format!(
        "<html>{}</html>",
        markdown_to_html(
            fs::read_to_string("readme.md").unwrap().as_str(),
            &ComrakOptions::default(),
        )
    ))
}

#[get("/favicon.ico")]
fn favicon() -> io::Result<Stream<File>> {
    File::open("favicon.ico").map(|file| Stream::from(file))
}

#[catch(404)]
fn not_found(req: &rocket::Request) -> JsonValue {
    dbg!(req);
    json!({
        "status": "error",
        "reason": "unknown"
    })
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

//#[catch(404)]
//fn not_found(req: &Request) -> String {
//    format!("I couldn't find '{}'. Try something else?", req.uri())
//}

#[catch(400)]
fn bad_request(req: &Request) -> String {
    dbg!(req);
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

//fn handle_400<'r>(req: &'r Request) -> Result<Response<'r>, Status> {
//    dbg!(&req);
//    let res = Custom(Status::NotFound, format!("404: {}", req.uri()));
//    res.respond_to(req)
//}

pub fn rocket() -> Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![imgtestform, index, favicon, pong, imgtestjson,],
        )
        .mount("/static", StaticFiles::from("static"))
        .register(catchers![not_found, internal_error, bad_request])
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
