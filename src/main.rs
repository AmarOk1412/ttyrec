extern crate wait_timeout;

use std::env;
use std::process::{Command, Stdio};
use std::io::{BufRead, Write, BufReader};
use wait_timeout::ChildExt;
use std::time::Duration;
use std::os::unix::io::IntoRawFd;

pub struct TTYRecorder {
    pub shell: String,
    pub window_id: String
}

impl TTYRecorder {

    pub fn get_shell() -> String {
        match env::var("SHELL") {
            Ok(value) => value,
            Err(_) => String::from("/bin/sh")
        }
    }

    pub fn get_windowid() -> String {
        match env::var("WINDOWID") {
            Ok(value) => value,
            Err(_) => String::from("")
        }
    }

    pub fn new() -> TTYRecorder {
        //Create the object
        TTYRecorder {
            shell: TTYRecorder::get_shell(),
            window_id: TTYRecorder::get_windowid()
        }
    }

    pub fn take_snapshot(&self) {
        println!("ID: {}", &self.window_id);
        let mut snapshot_child = Command::new("/bin/xwd")
        .arg("-id").arg(&self.window_id).arg("-out").arg("tty.xwd")
        .spawn()
        .expect("failed to take a screenshot");

        let ecode = snapshot_child.wait()
        .expect("failed to wait on child");

        assert!(ecode.success());

        let mut convert_child = Command::new("/bin/convert")
        .arg("tty.xwd").arg("tty.png")
        .spawn()
        .expect("failed to convert xwd file");

        let ecode = convert_child.wait()
        .expect("failed to wait on child");

        assert!(ecode.success());
    }

    pub fn record_child(&self) {
        let mut child = Command::new(&self.shell)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to launch a new shell");

        let one_sec = Duration::from_secs(1000);
        let mut status_code = None;

        let stdout = child.stdout.take().unwrap().into_raw_fd();
        let stderr = child.stderr.take().unwrap().into_raw_fd();
        let stdin = child.stdin.take().unwrap().into_raw_fd();

        println!("in {} ; out {} ; err {}", stdin, stdout, stderr);
        while status_code == None {
            status_code = match child.wait_timeout(one_sec).unwrap() {
                Some(status) => status.code(),
                None => None
            };
        }
    }
}

//Launch Medic
fn main() {
    let ttyrecorder = TTYRecorder::new();
    ttyrecorder.record_child();
}
