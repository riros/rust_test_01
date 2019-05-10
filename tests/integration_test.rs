// extern crate we're testing, same as any other code will do.


extern crate reqwest;
extern crate mime;
extern crate rocket;
extern crate file_diff;

// importing common module.
mod common;


//use mime_guess;

use reqwest::multipart::{Form, Part};



use std::fs::remove_file;
//use rocket::http::ext::IntoCollection;
//use rocket_http::ext::IntoCollection;
//use reqwest::multipart::*;
//use reqwest::multipart::Form;

//use file_diff::{diff_files};
//use std::fs::{File};


#[test]
fn test_integration_form() {
    // using common code. spawn service.
    common::setup();

    let thumbpath = "media/thumbnails/".to_string();


    println!("Removing files...");
    remove_file(thumbpath.clone() + "img.jpg").unwrap_or_default();
    remove_file(thumbpath.clone() + "img.png").unwrap_or_default();

    let form = Form::new()
        .part("text1", Part::text("mutipart multifile upload test"))
        .part("file1", Part::file("./tests/data/img.jpg").unwrap())
        .part("file2", Part::file("./tests/data/img.png").unwrap())
        .part("link1", Part::text(
            "https://static.ngs.ru/news/99/preview/7723aa3ee542cf82879e59b70613082c0bb790ee_800.jpg"
        ));


    let url = "http://localhost:8000/imgtest/v1";
    println!("Send request {}", &url);
    let service = reqwest::Client::new();
    let resp = service.post(url) // TODO dynamic port
        .multipart(form).send().unwrap();

    assert!(resp.status().is_success());

    // TODO files check
}

#[test]
fn test_integration_json() {
    // todo integration json
}
