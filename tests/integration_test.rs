// extern crate we're testing, same as any other code will do.
#[macro_use]
extern crate file_diff;
extern crate mime;
extern crate rocket;

use std::sync::Mutex;

// importing common module.
mod common;

use common::RocketLocalhostServer;
use core::borrow::BorrowMut;
use file_diff::diff_files;
use rayon::iter::IntoParallelIterator;
use reqwest::multipart::{Form, Part};
use std::fs::remove_file;
use std::fs::File;
use std::thread;

use std::process::{Child, Command};

use rayon::prelude::*;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_request_multipart_form_process() {
    common::setup();
    const PORT: &str = "8001";

    // ~~~!!! Only one rocket instance work. Dont use this in other tests.
    let mut rocket_server = RocketLocalhostServer::new(PORT);
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
        .post(format!("http://localhost:{}/imgtest/v1", PORT).as_str()) // TODO dynamic port
        .multipart(Form::new()
            .part("text1", Part::text("this part of text. must be skipped... "))
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

    //    rocket_server.print_info();
    rocket_server.shutdown("kill");

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
        remove_file(String::from(*res)).unwrap_or_default();
    }
}

#[test]
fn test_graceful_shutdown() {
    common::setup();
    const PORT: &str = "8002";
    // ~~~!!! Only one rocket instance work. Dont use this in other tests.
    let mut rocket_server = RocketLocalhostServer::new(PORT);

    let thr = thread::spawn(|| {
        let service = reqwest::Client::new();
        let resp = service
            .get(format!("http://localhost:{}/sleep/10", PORT).as_str()) // TODO dynamic port
            .send()
            .unwrap();

        assert!(resp.status().is_success());
    });

    sleep(Duration::new(1, 0));

    let mut pid = rocket_server.pid();

    let mut cmd = Command::new("kill");
    let mut signal = "-15".to_string();
    let proc = cmd.args(&[signal, pid.to_string()]).spawn();
    sleep(Duration::new(1, 0));

    let service = reqwest::Client::new();
    let resp = service
        .get(format!("http://localhost:{}/sleep/10", PORT).as_str()) // TODO dynamic port
        .send()
        .unwrap();

    assert!(resp.status().is_server_error());

    thr.join().unwrap();
    rocket_server.shutdown("kill");
}
