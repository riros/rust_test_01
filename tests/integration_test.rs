// extern crate we're testing, same as any other code will do.

extern crate file_diff;
extern crate mime;
extern crate rocket;

// importing common module.
mod common;

use common::RocketLocalhostServer;
use core::borrow::{Borrow, BorrowMut};
use file_diff::diff_files;
use rayon::iter::IntoParallelIterator;
use reqwest::multipart::{Form, Part};
use std::fs::remove_file;
use std::fs::File;

use rayon::prelude::*;

#[test]
fn test_request_multipart_form_process() {
    common::setup();

    // ~~~!!! Only one rocket instance work. Dont use this in other tests.
    let mut rocket_server = RocketLocalhostServer::new();
    // sleep 3 sec

    let ref imgs_v = vec![
        (
            "./tests/data/thumbnails/img.jpg",
            "media/thumbnails/img.jpg",
        ),
        (
            "./tests/data/thumbnails/img.png",
            "media/thumbnails/img.png",
        ),
        (
            "./tests/data/thumbnails/3a102f5862e95fc947e61fe70cc6ffda.jpg",
            "media/thumbnails/3a102f5862e95fc947e61fe70cc6ffda.jpg",
        ),
    ];

    println!("Removing files...");
    remove_fiels(imgs_v);

    let service = reqwest::Client::new();
    let resp = service
        .post("http://localhost:8002/imgtest/v1") // TODO dynamic port
        .multipart(Form::new()
            .part("text1", Part::text("mutipart multifile upload test"))
            .part("file1", Part::file("./tests/data/img.jpg").unwrap())
            .part("file2", Part::file("./tests/data/img.png").unwrap())
            .part(
                "link1",
                Part::text(
                    "http://www.lanzeva.ru/media/cache/3a/10/3a102f5862e95fc947e61fe70cc6ffda.jpg",
                ),
            ))
        .send()
        .unwrap();

    assert!(resp.status().is_success());

    rocket_server.print_info();
    rocket_server.shutdown();

    imgs_v.into_par_iter().for_each(|i| check_file(i));

    //remove_fiels();
}

fn check_file(file_pair: &(&'static str, &'static str)) {
    assert!(diff_files(
        File::open(file_pair.0).unwrap().borrow_mut(),
        File::open(file_pair.1).unwrap().borrow_mut(),
    ));
}

fn remove_fiels(v: &Vec<(&str, &str)>) {
    for (_, res) in v {
        remove_file(String::from(*res));
    }
}
