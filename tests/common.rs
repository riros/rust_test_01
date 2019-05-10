
use std::result::Result;
use std::result::Result::{Err, Ok};


pub fn start_server() -> Result<i8, &'static str> {
//    Err("server start failed")
    Ok(0)
}

pub fn setup() {
    // some setup code, like creating required files/directories, starting
    // servers, etc.
    assert_ne!(start_server(), Err("server start failed"));


}