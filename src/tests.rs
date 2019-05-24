use rocket;
use rocket::http::{Accept, ContentType, Header, MediaType, Method, Status};
use rocket::local::Client;

extern crate file_diff;
extern crate mime;
extern crate reqwest;

use reqwest::multipart::{Form, Part};
use serde_json;

use core::borrow::Borrow;
use core::borrow::BorrowMut;
use file_diff::diff_files;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use std::fs::File;
use std::fs::{read_to_string, remove_file};

use rayon::prelude::*;

// importing common module.

fn test<H>(
    method: Method,
    uri: &str,
    header: H,
    status: Status,
    req_body: String,
    resp_body: String,
) where
    H: Into<Header<'static>>,
{
    let rocket = super::rocket();
    //    rock.launch();

    let client = Client::new(rocket).unwrap();
    let mut response = client
        .req(method, uri)
        .header(header)
        .body(req_body)
        .dispatch();

    //    dbg!(&response.body_string());
    assert_eq!(response.status(), status);
    assert_eq!(response.body_string(), Some(resp_body));
}

fn remove_fiels() {
    const THUMBPATH: &str = "media/thumbnails/";
    remove_file(String::from(THUMBPATH) + "img.json.jpg").unwrap_or_default();
    remove_file(String::from(THUMBPATH) + "img.json.png").unwrap_or_default();
    remove_file(String::from(THUMBPATH) + "img.json.link.jpg").unwrap_or_default();
}

#[test]
fn test_integration_json() {
    //    test(
    //        Method::Get,
    //        "/message/99",
    //        Accept::JSON,
    //        Status::NotFound,
    //        "".to_string(),
    //        r##"{"reason":"Resource was not found.","status":"error"}"##.to_string(),
    //    );

    remove_fiels();
    test(
        Method::Post,
        "/imgtest/v1/",
        ContentType::JSON,
        Status::Ok,
        read_to_string("tests/data/request.json").unwrap(),
        r##""{\"status\":\"ok\"}""##.to_string(),
    );

    vec![
        ("./tests/data/img.jpg", "media/thumbnails/img.json.jpg"),
        ("./tests/data/img.png", "media/thumbnails/img.json.png"),
        (
            "./tests/data/3a102f5862e95fc947e61fe70cc6ffda.jpg",
            "media/thumbnails/img.json.link.jpg",
        ),
    ]
    .into_par_iter()
    .for_each(|i| check_file(i));
}

fn check_file(file_pair: (&'static str, &'static str)) {
    assert!(!diff_files(
        File::open(file_pair.0).unwrap().borrow_mut(),
        File::open(file_pair.1).unwrap().borrow_mut(),
    ));
}

#[test]
fn test_ping() {
    let body = "pong".to_string();
    test(
        Method::Get,
        "/ping",
        Accept::HTML,
        Status::Ok,
        "".to_string(),
        body.clone(),
    );
}

//
//#[test]
//fn test_hello() {
//    let person = Person { name: "Michael".to_string(), age: 80, };
//    let body = serde_json::to_string(&person).unwrap();
//    test(Method::Get, "/hello/Michael/80", Accept::JSON, Status::Ok, body.clone());
//    test(Method::Get, "/hello/Michael/80", Accept::Any, Status::Ok, body.clone());
//
//    // No `Accept` header is an implicit */*.
//    test(Method::Get, "/hello/Michael/80", ContentType::XML, Status::Ok, body);
//
//    let person = Person { name: "".to_string(), age: 99, };
//    let body = serde_json::to_string(&person).unwrap();
//    test(Method::Post, "/hello/99", ContentType::Plain, Status::Ok, body);
//}
//
//#[test]
//fn test_hello_invalid_content_type() {
//    let b = format!("<p>'{}' requests are not supported.</p>", MediaType::HTML);
//    test(Method::Get, "/hello/Michael/80", Accept::HTML, Status::NotFound, b.clone());
//    test(Method::Post, "/hello/80", ContentType::HTML, Status::NotFound, b);
//}
//
//#[test]
//fn test_404() {
//    let body = "<p>Sorry, '/unknown' is an invalid path! Try \
//                /hello/&lt;name&gt;/&lt;age&gt; instead.</p>";
//    test(Method::Get, "/unknown", Accept::JSON, Status::NotFound, body.to_string());
//}
