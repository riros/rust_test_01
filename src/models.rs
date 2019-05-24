use base64;
use rocket::http::hyper::mime::SubLevel;

use core::borrow::BorrowMut;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use raster::error::RasterError;
use raster::Image;
use reqwest::Client;
use rocket_contrib::json::{Json, JsonValue};
use std::fs::{remove_file, rename, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use tempfile::tempdir;
use url::Url;

use serde::{Deserialize, Serialize};

use raster::editor;

pub trait ImageInterface {
    fn from_url(url: &Url) -> Self;
    fn from_file(file_name: &String, file_type: &String, file_path: &PathBuf) -> Self;
    fn from_base64(file_name: &str, file_type: &str, base64_str: String) -> Self;
    //    fn get_image(&self) -> Result<Image, &'static str>;
}

/// Request
/// {
/// images:[
///     {
///         name:<String>,
///         image_type:<String>,
///         data: {
///             base64:<base64<String>>,
///             url: <String>,
///             path: <String>
///         }
///     }
/// ]
///
/// }
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    images: Vec<ImageStruct>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageStruct {
    name: String,
    image_type: String,
    data: Option<ImageSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSource {
    base64: Option<String>,
    url: Option<String>,
    path: Option<PathBuf>,
}

impl ImageSource {
    fn extract_filename_and_type_from_url(&self) -> Option<(String, String)> {
        match &self.url {
            Some(url) => {
                let surl = Url::parse(url.as_str()).unwrap(); // todo: catch errors
                let ref v1 = surl.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();

                let filename: &str = v1[v1.len() - 1];

                let sp: Vec<&str> = filename.split('.').collect();
                let url_ext = sp[sp.len() - 1];

                let ext = match url_ext {
                    "jpg" => SubLevel::Jpeg.as_str(),
                    "png" => SubLevel::Png.as_str(),
                    _ => {
                        panic!("Error parsing url file extension '{}'", url_ext);
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
            url: Some(url.clone().to_string()),
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
    pub fn make_thumbnail(&self, h: i32, w: i32) -> Result<(), RasterError> {
        let mut img = self.get_image().unwrap();
        //        let mut img = self.get_image().unwrap().borrow_mut();
        editor::resize(img.borrow_mut(), w, h, raster::editor::ResizeMode::Fit)?;
        raster::save(
            img.borrow_mut(),
            Path::new("media/thumbnails")
                .join(&self.name)
                .to_str()
                .unwrap(),
        )?;
        Ok(())
    }

    pub fn get_image<'a>(&self) -> Result<Image, RasterError> {
        fn image_from_reader<'a, R: ?Sized>(
            reader: &mut R,
            image_type: &str,
        ) -> Result<Image, RasterError>
        where
            R: Read,
        {
            let rand_sring: String = thread_rng().sample_iter(&Alphanumeric).take(8).collect();
            let tmpdir = tempdir().unwrap();
            let file_path = tmpdir.path().join(rand_sring);
            //            dbg!(&file_path);
            let mut tmpfile = File::create(&file_path).unwrap();
            io::copy(reader, &mut tmpfile).unwrap();
            drop(tmpfile);

            let new_file_path = &rename_file(&file_path, image_type);

            //            dbg!(new_file_path);

            //            let mut img = raster::open(new_file_path);

            let i = raster::open(new_file_path)?;
            remove_file(new_file_path)?;
            Ok(i)
        }

        fn rename_file(pathbuf: &PathBuf, ext: &str) -> String {
            let new_path = format!("{}.{}", &pathbuf.to_str().unwrap(), ext);
            rename(pathbuf.to_str().unwrap(), &new_path).unwrap();
            new_path
        }

        let imgsrc: &ImageSource =
//            &self.data.unwrap();
            match &self.data {
                Some(d) => { d }
                None => {
                    dbg!(&self);
                    panic!("No data field in ImageSource")
                }
            };

        match &imgsrc.path {
            Some(pathbuf) => {
                let new_filename = rename_file(pathbuf, &self.image_type.to_string());
                //                dbg!(&new_filename);
                let retimage = raster::open(&new_filename).unwrap();
                Ok(retimage)
            }
            None => match &imgsrc.url {
                Some(url) => {
                    let mut resp = Client::new().get(url.as_str()).send().unwrap();

                    match image_from_reader(&mut resp, &self.image_type.as_str()) {
                        Ok(im) => Ok(im),
                        Err(e) => Err(e),
                    }
                }
                None => match &imgsrc.base64 {
                    Some(b64) => {
                        let mut reader = base64::decode(b64).unwrap();
                        let mut reader: &[u8] = reader.as_mut();
                        match image_from_reader(&mut reader, &self.image_type.as_str()) {
                            Ok(im) => Ok(im),
                            Err(e) => Err(e),
                        }
                    }
                    None => Err(RasterError::Unexpected),
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
            name: filename,
            image_type: filetype,
            data: Some(ims),
        }
    }

    fn from_file(_file_name: &String, _file_type: &String, _file_path: &PathBuf) -> ImageStruct {
        let ims: ImageSource = ImageInterface::from_file(_file_name, _file_type, _file_path);
        ImageStruct {
            name: _file_name.clone(),
            image_type: _file_type.clone(),
            data: Some(ims),
        }
    }

    fn from_base64(file_name: &str, _file_type: &str, _base64_str: String) -> ImageStruct {
        panic!("Not Implemented!");
    }
}
