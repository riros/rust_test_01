//use std::result::Result;
//use std::result::Result::{Err, Ok};

use std::process::{Child, Command};

use std::thread::sleep;
use std::time::Duration;

pub fn setup() {

    // some setup code, like creating required files/directories, starting
    // servers, etc.
}

#[derive(Debug)]
pub enum RocketServerStatus {
    Running,
    //    GracefulShutdownProcess,
    Shutdown,
}

pub struct RocketLocalhostServer {
    cmd: Command,
    proc: Result<Child, &'static str>,
    status: RocketServerStatus,
}

impl Default for RocketLocalhostServer {
    fn default() -> Self {
        let mut cmd = Command::new("cargo");
        cmd.env("ROCKET_PORT", "8002");
        let proc = cmd.args(&["run"]).spawn();
        let (status, child) = match proc {
            Ok(se) => (RocketServerStatus::Running, Ok(se)),

            Err(_e) => (RocketServerStatus::Shutdown, Err("Spawn process failed.")),
        };
        sleep(Duration::new(3, 0));
        //        child.unwrap().kill();

        RocketLocalhostServer {
            cmd: cmd,
            proc: child,
            status: status,
        }
    }
}

impl RocketLocalhostServer {
    pub fn new() -> Self {
        RocketLocalhostServer::default()
    }

    pub fn shutdown(&mut self) {
        match &mut self.proc {
            Ok(p) => p.kill().unwrap(),
            Err(_) => (),
        }
    }

    //    fn check_running(&mut self) -> bool {
    //        let mut system = sysinfo::System::new();
    //        system.refresh_all();
    //        let id: i32 = self.pid().to_i32().unwrap();
    //        let p: Option<_> = system.get_process(id);
    //        dbg!(&p);
    //        p.is_some()
    //    }
    //    fn pid(&self) -> u32 {
    //        match &self.proc {
    //            Ok(c) => c.id(),
    //            Err(_) => 0,
    //        }
    //    }
    pub fn print_info(&self) {
        dbg!(&self.status, &self.cmd);
    }
}
