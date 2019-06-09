#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin, custom_attribute)]
#[macro_use]
extern crate rocket;
extern crate base64;
extern crate graceful;
extern crate lazy_static;
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
use rocket::response::Stream;
use rocket::{Data, Request, Response, Rocket};

use rocket::http::uri::Origin;

use rocket_contrib::serve::StaticFiles;
use std::io::{self, Cursor};

mod functions;
mod models;

use functions::*;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use std::fs::File;

use comrak::{markdown_to_html, ComrakOptions};
use rocket::response::content;
use std::fs;

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use graceful::SignalGuard;

static STOP: AtomicBool = AtomicBool::new(false);

use rocket::fairing::{Fairing, Info, Kind};
use std::thread::sleep;

static WORK_COUNT: AtomicUsize = AtomicUsize::new(0);
static WORKS_ESTIMATE: AtomicUsize = AtomicUsize::new(1);
static SHUTDOWN: AtomicBool = AtomicBool::new(false);


#[derive(Default)]
struct Watchdog {}

impl Fairing for Watchdog {
    fn info(&self) -> Info {
        Info {
            name: "Work watchdog",
            kind: Kind::Request | Kind::Response,
        }
    }

    fn on_request(&self, request: &mut Request, _: &Data) {
        if SHUTDOWN.load(Ordering::SeqCst) {
            WORK_COUNT.fetch_add(1, Ordering::SeqCst);
            request.set_uri(Origin::parse("/ping").unwrap());
        } else {
            WORK_COUNT.fetch_add(1, Ordering::SeqCst);
            WORKS_ESTIMATE.store(WORK_COUNT.load(Ordering::SeqCst), Ordering::SeqCst)
        }

        //        }
    }

    fn on_response(&self,
                   _: &Request
                   , response: &mut Response) {
        if SHUTDOWN.load(Ordering::SeqCst) {
            if WORKS_ESTIMATE.load(Ordering::SeqCst) < WORK_COUNT.load(Ordering::SeqCst) {
                let body = format!("Server Busy... Shutting down... Try later.");
                response.set_status(Status::ServiceUnavailable);
                response.set_header(ContentType::Plain);
                response.set_sized_body(Cursor::new(body));

                WORK_COUNT.fetch_sub(1, Ordering::SeqCst);
            } else {
                WORKS_ESTIMATE.store(WORK_COUNT.fetch_sub(1, Ordering::SeqCst), Ordering::SeqCst)
            }
        } else {
            WORK_COUNT.fetch_sub(1, Ordering::SeqCst);
        }
    }
}

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

#[get("/sleep/<sec>")]
fn hsleep(sec: u64) -> content::Plain<String> {
    sleep(Duration::new(sec, 0));
    content::Plain("done".to_string())
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
        "reason": "Resource was not found."
    })
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(400)]
fn bad_request(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

pub fn rocket() -> Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![imgtestform, index, favicon, pong, imgtestjson, hsleep,],
        )
        .mount("/static", StaticFiles::from("static"))
        .attach(Watchdog::default())
        .register(catchers![not_found, internal_error, bad_request])
}

pub fn main() {
    thread::spawn(move || {
//        println!("Rocket server start...");
        rocket().launch();
//        println!("Rocket end...");
    });

    let signal_guard = SignalGuard::new();

    let watchdog = thread::spawn(|| {
//        println!("Worker thread started. Type Ctrl+C to stop.");
        while !STOP.load(Ordering::Acquire) {
            //            println!("still working...");
            thread::sleep(Duration::from_millis(5000));
        }
        while WORK_COUNT.load(Ordering::SeqCst) > 0 {
            //            println!("waiting works done... Sleep 500ms...");
            thread::sleep(Duration::from_millis(50));
        }

//        println!("Rocket shutting down .");
    });

    signal_guard.at_exit(move |sig| {
        println!("Signal {} received.", sig);
        STOP.store(true, Ordering::Release);
        SHUTDOWN.store(true, Ordering::Release);
        watchdog.join().unwrap();
    });
}
