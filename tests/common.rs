//use std::result::Result;
//use std::result::Result::{Err, Ok};

pub fn setup() {

    // some setup code, like creating required files/directories, starting
    // servers, etc.
}

pub enum RocketServerStatus {
    Running,
    Shutdown,
    Error,
}

pub struct RocketServer {
    pid: u8,
    status: RocketServerStatus,
}

impl Default for RocketServer {
    fn default() -> Self {
        // todo
        RocketServer {
            pid: 0,
            status: RocketServerStatus::Running,
        }
    }
}

impl RocketServer {
    pub fn spawn_rocket() {
        unimplemented!()
    }
    pub fn shutdown_rocket() {
        unimplemented!()
    }
    pub fn check_running() -> bool {
        unimplemented!()
    }
}
