//use multipart::mock::StdoutTee;
use multipart::server::save::Entries;
use multipart::server::save::SaveResult::*;
use multipart::server::save::SavedData;
use multipart::server::{FieldHeaders, Multipart};

use rocket::http::hyper::mime::{Mime, SubLevel, TopLevel};
use rocket::Data;
use std::io::{self, Write};
use std::fs;
use url::Url;
use rayon::prelude::*;

use crate::models::{ImageInterface, ImageStruct};
use std::path::PathBuf;

use raster::Image;


pub fn process_upload(boundary: &str, data: Data) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();

    // saves all fields, any field longer than 10kB goes to a temporary directory
    // Entries could implement FromData though that would give zero control over
    // how the files are saved; Multipart would be a good impl candidate though
    match Multipart::with_body(data.open(), boundary).save().temp() {
        Full(entries) => process_entries(entries, &mut out)?,
        Partial(partial, reason) => {
            writeln!(out, "Request partially processed: {:?}", reason)?;
            if let Some(field) = partial.partial {
                writeln!(out, "Stopped on field: {:?}", field.source.headers)?;
            }

            process_entries(partial.entries, &mut out)?
        }
        Error(e) => return Err(e),
    }

    Ok(out)
}

pub fn process_entries(entries: Entries, out: &mut Vec<u8>) -> io::Result<()> {
    {
        //        let stdout = io::stdout();
        //        let tee = StdoutTee::new(&mut out, &stdout);

        let mut imgs: Vec<ImageStruct> = vec![];


        for (_, v) in entries.fields {
            for sf in v {
//                println!("{:?}", &sf);

                let hdrs: FieldHeaders = sf.headers;
//                println!("{:?}", &hdrs);
                let file_name = &hdrs.filename.unwrap_or_default();
//                let file_name: &String = &"OK".to_string();
                let sd: SavedData = sf.data;

                match &hdrs.content_type {
                    Some(ref omime) => {
                        match omime {
                            Mime(TopLevel::Image, SubLevel::Jpeg, _) => {
                                push_image_struct_from_file(&mut imgs,
                                                            file_name,
                                                            &sd, SubLevel::Jpeg,
                                );
                            }
                            Mime(TopLevel::Image, SubLevel::Png, _) => {
                                push_image_struct_from_file(&mut imgs,
                                                            file_name,
                                                            &sd, SubLevel::Png,
                                );
                            }

                            _ => eprintln!("{:?}", "Mime type not Image/(jpg|png)"),
                        }
                    }
                    _ => {
                        println!("No content type. Process text as url");
                        match sd {
                            SavedData::Text(txt) => {
                                let url_parser = Url::parse(txt.as_str());
                                match url_parser {
                                    Ok(_url) => {
                                        imgs.push(
                                            ImageInterface::from_url(&_url)
                                        );
                                    }
                                    Err(p) => eprintln!(
                                        "unsupported text '{}' with error: '{}'",
                                        txt,
                                        p.to_string()
                                    ),
                                }
                            }
                            SavedData::File(_pathbuf, _fsize) => {
                                println!("TODO: implement read file from server path.");
                            }
                            SavedData::Bytes(_) => {
                                eprintln!("TODO: implement read bytes from data field.")
                            }
                        }
                    }
                }
            }
        }

        dbg!(&imgs);

        println!("threads work: Start");
        // Start rayon multiprocess
//        imgs.par_iter().for_each(|i| make_thumbnails(&i, 100, 100));
        for i in imgs {
            make_thumbnails(&i, 100, 100)
        }
        println!("threads work: Done");


//        entries.write_debug(tee)?;
    }

    writeln!(out, "")
}


/// В задаче ничего не сказано о том, как отдавать превью. Поэтому, будем просто сохранять их в каталог
fn make_thumbnails(imgstrct: &ImageStruct, w: u16, h: u16) {
    let img: Image = imgstrct.get_image().unwrap();
//    dbg!(img);
}


fn push_image_struct_from_file(
    to_vector: &mut Vec<ImageStruct>,
    file_name: &String,
    sd: &SavedData, sl: SubLevel,
) {
    println!("processing {}.. ", sl.to_string());
    match sd {
        SavedData::File(pathbuf, size) =>
            {
                let new_filename = format!("{}.{}", &pathbuf.to_str().unwrap(), sl.to_string());

                fs::rename(
                    pathbuf.to_str().unwrap(),
                    &new_filename,
                ).unwrap();

                to_vector.push(
                    ImageInterface::from_file(
                        &file_name,
                        &sl.to_string(),
                        &PathBuf::from(&new_filename),
                    )
                )
            }
        _ => {
            panic!("mismatch mime type {:?} and SavedData '{:?}' content! ", sl, sd)
        }
    }
}