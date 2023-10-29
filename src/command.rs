use mpvipc::*;
use std::sync::mpsc::channel;
use serde_json::{Value, Result,from_value};
use std::path::Path;
use std::process::Command;

use crate::models::{PlayMessage, SeekMessage, SetVolumeMessage};

const MPVSOCKET: &str = "/tmp/mpvsock"; //TODO: config
const DUMMY: bool = false;
//const DEFARGS: Vec<&str> = ["--input-ipc-server=/tmp/mpvsock", "--idle", "--force-window"];

pub fn play(command: Value) {
    let command: PlayMessage = from_value(command).unwrap();
    let mut commandstring: Vec<String>  = Vec::new();

    match command.url {
        Some(val) => commandstring.push(val),
        None => (),
    }

    //Support this later I assume this is so that the fcast can send the actual file itself
    match command.content {
        Some(_val) => println!("TODO: Not currently supported"),
        None => (),
    }

    match command.time {
        Some(val) => commandstring.push(val.to_string()),
        None => panic!("Time is necessary payload."),
    }

    println!("execute play");
    let exists = checksock();
    println!("exists is {}", exists);
    if exists == false && DUMMY == false {
        newinstance(commandstring);
    } else {
        println!("change the playing video");
    }
    
}

pub fn pause() {
    if checksock() == false {
        panic!("Mpv is not running, it may have crashed")
    }
    if DUMMY == false {
        let mpv = Mpv::connect(MPVSOCKET).unwrap();
        mpv.set_property("pause", true).expect("Error pausing");
    } else {
        println!("exec pause");
    }
}

pub fn resume() {
    if !checksock() {
        panic!("Mpv is not running, it may have crashed")
    }
    if DUMMY == false {
        let mpv = Mpv::connect(MPVSOCKET).unwrap();
        mpv.set_property("pause", false).expect("Error resuming");
    } else {
        println!("exec resume");
    }
}

pub fn stop() {
    if !checksock() {
        panic!("Mpv is not running, it may have crashed")
    }
    if DUMMY == false {
        let mpv = Mpv::connect(MPVSOCKET).unwrap();
        //mpv.set_property("pause", false).expect("Error resuming");
        //Should this kill MPV?
        mpv.kill().unwrap();
    } else {
        println!("exec stop");
    }
}

pub fn seek(command: Value) {
    if !checksock() {
        panic!("Mpv is not running, it may have crashed")
    }
    if DUMMY == false {
        let mpv = Mpv::connect(MPVSOCKET).unwrap();
        let command: SeekMessage = from_value(command).unwrap();
        mpv.set_property("timestamp", command.time as f64).expect("Error resuming");
    } else {
        println!("exec seek");
    }
}

pub fn setvol(command: Value) {
    if !checksock() {
        panic!("Mpv is not running, it may have crashed")
    }
    if DUMMY == false {
        let mpv = Mpv::connect(MPVSOCKET).unwrap();
        let command: SetVolumeMessage = from_value(command).unwrap();
        mpv.set_property("volume", command.volume as f64).expect("Error resuming");
    } else {
        println!("exec setvol");
    }
}

//TODO: set idle and force-window via config
fn newinstance(args: Vec<String>) {
    //println!("DUMMY COMMAND mpv {} --start={} {}", args[0], args[1], args[2]);
    let uri = &args[0];
    let time =&args[1];
    let seek: String = "--start=".to_owned() + time;
    //let def = &args[2];
    if DUMMY == false {
        let output =
        Command::new("mpv")
                .args(["--input-ipc-server=/tmp/mpvsock", "--idle", "--force-window=immediate"])
                .arg(uri)
                .arg(seek)
                .spawn()
                .expect("Failed to execute command");
        //println!("mpv is {:?}, output is {:?}", std::str::from_utf8(&output.stderr).unwrap(), std::str::from_utf8(&output.stdout).unwrap());
    } else {
        println!("DUMMY COMMAND mpv {} --start={}", uri, time);
    }
}

fn checksock() -> bool {
    let path = Path::new(MPVSOCKET);

    return path.exists()
}

