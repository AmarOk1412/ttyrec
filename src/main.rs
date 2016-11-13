extern crate clap;

use std::env;
use std::process::Command;
use std::time::Duration;
use std::thread;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use clap::{Arg, App};

pub struct TTYRecorder {
    pub shell: String,
    pub window_id: String,
    pub delay_screenshot: u64,
    pub delay_gif: u64,
    pub outname: String,
    pub format: String
}

impl TTYRecorder {

    pub fn new(snap_delay: u64, gif_delay: u64, outname: String, format: String) -> TTYRecorder {
        //Create the object
        TTYRecorder {
            shell: TTYRecorder::get_shell(),
            window_id: TTYRecorder::get_windowid(),
            delay_screenshot: snap_delay,
            delay_gif: gif_delay,
            outname: outname,
            format: format
        }
    }

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

    pub fn take_snapshot(window_id: String, cpt: i64) {
        let path_xwd = format!("{:020}.xwd", cpt);
        let mut snapshot_child = Command::new("/bin/xwd")
        .arg("-id").arg(window_id).arg("-out").arg(&path_xwd)
        .spawn()
        .expect("failed to take a screenshot");

        let ecode = snapshot_child.wait()
        .expect("failed to wait on child");

        assert!(ecode.success());
    }

    pub fn convert_to_output(&self) {
        println!("Creating output...");
        let delay = format!("{}", &self.delay_gif);
        let out_file = format!("{}.{}", &self.outname, &self.format);
        let mut convert_child = Command::new("/bin/convert")
        .arg("-delay").arg(delay)
        .arg("*.xwd").arg(&out_file)
        .spawn()
        .expect("failed to remove *.xwd");

        let ecode = convert_child.wait()
        .expect("failed to wait on child");

        assert!(ecode.success());
        println!("done: {}", out_file);
        println!("Removing useless files");

        let mut cpt = 0;
        loop {
            let xwd_file = format!("{:020}.xwd", cpt);
            let xwd_exists = Path::new(&xwd_file).exists();
            if xwd_exists {
                fs::remove_file(&xwd_file).expect("Can't remove file");
            } else {
                break;
            }
            cpt = cpt + 1;
        }
    }

    pub fn record_child(&self) {
        let mut clear_child = Command::new("/bin/clear")
        .spawn()
        .expect("failed to clear terminal");

        let ecode = clear_child.wait()
        .expect("failed to wait on child");

        assert!(ecode.success());
        thread::sleep(Duration::from_millis(500));

        if self.format == String::from("log") {
            let outfile = format!("{}.log", self.outname);
            let mut child = Command::new("/bin/script")
            .arg("--timing=timing.log")
            .arg(outfile)
            .spawn()
            .expect("failed to launch script command");
            let ecode = child.wait()
            .expect("failed to wait on child");
            assert!(ecode.success());

        } else {
            let mut child = Command::new(&self.shell)
            .spawn()
            .expect("failed to launch a new shell");

            let window_id = self.window_id.clone();
            let delay = self.delay_screenshot;

            let lock = Arc::new(AtomicBool::new(true));
            let lock_clone = lock.clone();
            let snap_thread = thread::spawn(move || {
                let mut cpt = 0;
                while lock_clone.fetch_and(true, Ordering::SeqCst) {
                    TTYRecorder::take_snapshot(window_id.clone(), cpt);
                    cpt = cpt + 1;
                    thread::sleep(Duration::from_millis(delay));
                }
            });

            let ecode = child.wait()
            .expect("failed to wait on child");
            assert!(ecode.success());
            lock.store(false, Ordering::Relaxed);
            match snap_thread.join() {
                Ok(_) => return,
                Err(e) => panic!(e),
            };
        }
    }
}


fn main() {
    let matches = App::new("ttyrec")
                  .version("0.1")
                  .about("Create gif from tty input")
                  .arg(Arg::with_name("snap-delay")
                  .short("sd")
                  .long("snap-delay")
                  .help("Change delay between 2 snapshot")
                  .takes_value(true))
                  .arg(Arg::with_name("out-delay")
                  .short("od")
                  .long("out-delay")
                  .help("Change delay between 2 frame for the output file")
                  .takes_value(true))
                  .arg(Arg::with_name("base-filename")
                  .short("bf")
                  .long("base-filename")
                  .help("Change output name")
                  .takes_value(true))
                  .arg(Arg::with_name("video")
                  .short("v")
                  .long("video")
                  .help("Add a tty.mp4"))
                  .arg(Arg::with_name("text")
                  .short("t")
                  .long("text")
                  .help("Write a script file with timing.txt"))
                  .get_matches();

    let snap_delay = matches.value_of("snap-delay").unwrap_or("250").parse::<u64>().unwrap();
    let mut gif_delay = matches.value_of("out-delay").unwrap_or("30").parse::<u64>().unwrap();
    let outname = String::from(matches.value_of("base-filename").unwrap_or("tty"));

    let mut format = String::from("gif");
    if matches.is_present("video") {
        format = String::from("mpeg");
        if !matches.is_present("out-delay") {
            gif_delay = 5;
        }
    }
    if matches.is_present("text") {
        format = String::from("log");
    }

    let end_format = format.clone();
    let ttyrecorder = TTYRecorder::new(snap_delay, gif_delay, outname, format);
    ttyrecorder.record_child();
    if end_format != String::from("log") {
        ttyrecorder.convert_to_output();
    }
}
