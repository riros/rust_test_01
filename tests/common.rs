use std::process::{Child, Command};

use std::thread::sleep;

//use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn setup() {

    // some setup code, like creating required files/directories, starting
    // servers, etc.
}

//#[derive(Debug)]
//pub enum RocketServerStatus {
//    Running,
//    Shutdown,
//}

pub struct RocketLocalhostServer {
//    cmd: Command,
    proc: Result<Child, &'static str>,
//    status: RocketServerStatus,
}

impl RocketLocalhostServer {
    pub fn new(port: &str) -> Self {
        let mut cmd = Command::new("cargo");
        cmd.env("ROCKET_PORT", port);
        let proc = cmd.args(&["run"]).spawn();
        let child = match proc {
            Ok(se) => Ok(se),

            Err(_e) =>  Err("Spawn process failed."),
        };
        sleep(Duration::new(5, 0));
        //        child.unwrap().kill();

        RocketLocalhostServer {
//            cmd: cmd,
            proc: child,
//            status: status,
        }
    }

    pub fn shutdown(&mut self, method: &str) {
        match &mut self.proc {
            Ok(p) => match method {
                "kill" => p.kill().unwrap(),
                //                "term" => p.terminate().unwrap(),
                _ => panic!("unknown process method"),
            },
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
    pub fn pid(&self) -> u32 {
        match &self.proc {
            Ok(c) => c.id(),
            Err(_) => 0,
        }
    }
    //    pub fn print_info(self) {
    //        dbg!(self.status);
    //        dbg!(self.cmd);
    //    }
}
