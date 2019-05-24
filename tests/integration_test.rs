// extern crate we're testing, same as any other code will do.

extern crate file_diff;
extern crate mime;
extern crate rocket;

// importing common module.
mod common;
use common::RocketLocalhostServer;
use core::borrow::BorrowMut;
use file_diff::diff_files;
use reqwest::multipart::{Form, Part};
use std::fs::remove_file;
use std::fs::File;

#[test]
fn test_request_multipart_form_process() {
    common::setup();

    // ~~~!!! Only one rocket instance work. Dont use this in other tests.
    let mut rocket_server = RocketLocalhostServer::new();
    // sleep 3 sec

    println!("Removing files...");
    remove_fiels();

    let form = Form::new()
        .part("text1", Part::text("mutipart multifile upload test"))
        .part("file1", Part::file("./tests/data/img.jpg").unwrap())
        .part("file2", Part::file("./tests/data/img.png").unwrap())
        .part(
            "link1",
            Part::text(
                "http://www.lanzeva.ru/media/cache/3a/10/3a102f5862e95fc947e61fe70cc6ffda.jpg",
            ),
        );

    let url = "http://localhost:8002/imgtest/v1";
    println!("Send request {}", &url);
    let service = reqwest::Client::new();
    let resp = service
        .post(url) // TODO dynamic port
        .multipart(form)
        .send()
        .unwrap();

    assert!(resp.status().is_success());

    rocket_server.print_info();
    rocket_server.shutdown();

    //         TODO files check
    assert!(!diff_files(
        File::open("./tests/data/img.jpg").unwrap().borrow_mut(),
        File::open("media/thumbnails/img.jpg").unwrap().borrow_mut(),
    ));
    assert!(!diff_files(
        File::open("./tests/data/img.png").unwrap().borrow_mut(),
        File::open("media/thumbnails/img.png").unwrap().borrow_mut(),
    ));
    assert!(!diff_files(
        File::open("./tests/data/3a102f5862e95fc947e61fe70cc6ffda.jpg")
            .unwrap()
            .borrow_mut(),
        File::open("media/thumbnails/3a102f5862e95fc947e61fe70cc6ffda.jpg")
            .unwrap()
            .borrow_mut(),
    ));

    remove_fiels();
}

fn remove_fiels() {
    const THUMBPATH: &str = "media/thumbnails/";
    remove_file(String::from(THUMBPATH) + "img.jpg").unwrap_or_default();
    remove_file(String::from(THUMBPATH) + "img.png").unwrap_or_default();
    remove_file(String::from(THUMBPATH) + "3a102f5862e95fc947e61fe70cc6ffda.jpg")
        .unwrap_or_default();
}
