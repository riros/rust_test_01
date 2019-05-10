use base64;
use rocket::http::hyper::mime::SubLevel;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use raster::Image;
use reqwest::Client;
use std::env::split_paths;
use std::fs::{remove_file, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use tempfile::{tempdir, tempfile};
use url::Url;

pub trait ImageInterface {
    fn from_url(url: &Url) -> Self;
    fn from_file(file_name: &String, file_type: &String, file_path: &PathBuf) -> Self;
    fn from_base64(file_name: &str, file_type: &str, base64_str: String) -> Self;
    //    fn get_image(&self) -> Result<Image, &'static str>;
}

#[derive(Debug)]
pub struct Images {
    images: Vec<ImageStruct>,
}

#[derive(Debug)]
pub struct ImageStruct {
    name: String,
    image_type: String,
    data: Option<ImageSource>,
}

#[derive(Debug)]
pub struct ImageSource {
    base64: Option<String>,
    url: Option<Url>,
    path: Option<PathBuf>,
}

impl ImageSource {
    fn extract_filename_and_type_from_url(&self) -> Option<(String, String)> {
        match &self.url {
            Some(url) => {
                let ref v1 = url.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();

                let filename: &str = v1[v1.len() - 1];

                let sp: Vec<&str> = filename.split('.').collect();
                let mut ext: &str = sp[sp.len() - 1].clone();

                match ext {
                    "jpg" => ext = SubLevel::Jpeg.as_str(),
                    "png" => ext = SubLevel::Png.as_str(),
                    _ => {
                        println!("Error parsing url file extension '{}'", ext);
                    }
                };
                Some((filename.to_string(), ext.to_string()))
            }
            None => None,
        }
    }
}

impl Default for ImageStruct {
    fn default() -> Self {
        ImageStruct {
            image_type: SubLevel::Jpeg.to_string(),
            name: "unnamed.jpg".to_string(),
            data: None,
        }
    }
}

impl ImageInterface for ImageSource {
    fn from_url(url: &Url) -> ImageSource {
        ImageSource {
            url: Some(url.clone()),
            base64: None,
            path: None,
        }
    }

    fn from_file(file_name: &String, file_type: &String, file_path: &PathBuf) -> ImageSource {
        ImageSource {
            url: None,
            base64: None,
            path: Some(file_path.clone()),
        }
    }

    fn from_base64(file_name: &str, file_type: &str, base64_str: String) -> ImageSource {
        ImageSource {
            url: None,
            base64: Some(base64_str),
            path: None,
        }
    }
}

impl ImageStruct {
    pub fn get_image(&self) -> Result<Image, &'static str> {
        fn image_from_reader<R: ?Sized>(reader: &mut R) -> raster::Image
        where
            R: Read,
        {
            let rand_sring: String = thread_rng().sample_iter(&Alphanumeric).take(8).collect();
            let tmpdir = tempdir().unwrap();
            let mut file_path = tmpdir.path().join(rand_sring);
            dbg!(&file_path);
            let mut tmpfile = File::open(&file_path).unwrap();

            io::copy(reader, &mut tmpfile).unwrap();

            let img = raster::open(&file_path.to_str().unwrap()).unwrap();
            drop(tmpfile);
            remove_file(file_path).unwrap();
            img
        }
        let imgsrc = &self.data.unwrap();

        match &self.path {
            Some(pathbuf) => {
                dbg!(&pathbuf);
                Ok(raster::open(pathbuf.to_str().unwrap()).unwrap())
            }
            None => match &self.url {
                Some(url) => {
                    let mut resp = Client::new().get(url.as_str()).send().unwrap();
                    Ok(image_from_reader(&mut resp))
                }
                None => match &self.base64 {
                    Some(b64) => {
                        let mut reader = base64::decode(b64).unwrap();
                        let mut reader: &[u8] = reader.as_mut();
                        Ok(image_from_reader(&mut reader))
                    }
                    None => Err("No image data"),
                },
            },
        }
    }
}

impl ImageInterface for ImageStruct {
    fn from_url(url: &Url) -> ImageStruct {
        let ims: ImageSource = ImageInterface::from_url(url);

        let (filename, filetype) = ims.extract_filename_and_type_from_url().unwrap();

        ImageStruct {
            name: filename.clone(),
            image_type: filetype.clone(),
            data: Some(ims),
        }
    }

    fn from_file(file_name: &String, file_type: &String, file_path: &PathBuf) -> ImageStruct {
        let ims: ImageSource = ImageInterface::from_file(file_name, file_type, file_path);
        ImageStruct {
            name: file_name.clone(),
            image_type: file_type.clone(),
            data: Some(ims),
        }
    }

    fn from_base64(file_name: &str, file_type: &str, base64_str: String) -> ImageStruct {
        panic!("Not Implemented!");
        ImageStruct::default()
    }

    //    fn get_image(&self) -> Result<Image, &'static str> {
    //        match &self.data {
    //            Some(imsrc) => imsrc.get_image(),
    //            None => Err("No data")
    //        }
    //    }
}

//impl TimageObj for imageObj{
//
//}
