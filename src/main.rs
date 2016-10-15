extern crate wait_timeout;

use std::env;
use std::process::Command;
use wait_timeout::ChildExt;
use std::time::Duration;

pub struct TTYRecorder {
    pub shell: String
}

impl TTYRecorder {

    pub fn get_shell() -> String {
        match env::var("SHELL") {
            Ok(value) => value,
            Err(_) => String::from("/bin/sh")
        }
    }

    pub fn new() -> TTYRecorder {
        //Create the object
        TTYRecorder {
            shell: TTYRecorder::get_shell()
        }
    }

    pub fn record_child(&self) {
        let mut child = Command::new(&self.shell)
        .spawn()
        .expect("failed to launch a new shell");

        let one_sec = Duration::from_secs(1000);
        let mut status_code = None;

        while status_code == None {
            status_code = match child.wait_timeout(one_sec).unwrap() {
                Some(status) => status.code(),
                None => None
            };
            //TODO get input/output
            println!("Shell Running");
        }

        println!("Shell Finished {}", status_code.unwrap());
    }
}

//Launch Medic
fn main() {
    let ttyrecorder = TTYRecorder::new();
    ttyrecorder.record_child();
}
