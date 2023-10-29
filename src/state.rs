use crate::models::{PlayState, PlaybackUpdateMessage, VolumeUpdateMessage};
use mpvipc::*;
use std::io::Write;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::path::Path;
use std::{thread, time};

const MPVSOCKET: &str = "/tmp/mpvsock"; //TODO: config

pub fn updatestate(mut stream: TcpStream) {
    loop {
        //thread::sleep(time::Duration::from_millis(250));
        let exists = checksock();
        if exists == true {
            let mut mpv = Mpv::connect(MPVSOCKET).unwrap();
            //let mut idle = true;
            let mut pause = false;
            let mut playback_time = std::f64::NAN;
            //let mut duration = std::f64::NAN;
            mpv.observe_property(1, "idle").unwrap();
            mpv.observe_property(2, "pause").unwrap();
            mpv.observe_property(3, "playback-time").unwrap();
            loop {
                let event = mpv.event_listen().unwrap();
                match event {
                    Event::PropertyChange { id: _, property } => match property {
                        Property::Path(Some(_)) => (),
                        Property::Path(None) => (),
                        Property::Pause(value) => pause = value,
                        Property::PlaybackTime(Some(value)) => playback_time = value,
                        Property::PlaybackTime(None) => playback_time = std::f64::NAN,
                        Property::Duration(Some(_)) => (),
                        Property::Duration(None) => (),
                        Property::Metadata(Some(_)) => (),
                        Property::Metadata(None) => (),
                        Property::Unknown { name: _, data: _ } => (),
                    },
                    //Event::Shutdown => return Ok(()),
                    Event::Unimplemented => panic!("Unimplemented event"),
                    _ => (),
                }

                println!("mpv socket found");
                let idle: bool = mpv.get_property("idle").unwrap();
                let mut playback_state = 0;
                if idle == true {
                    let response = PlaybackUpdateMessage::new(0, 0);
                } else {
                    if pause == true {
                        playback_state = 2;
                    } else {
                        playback_state = 1;
                    }
                }
                let response = PlaybackUpdateMessage::new(playback_time as u64, playback_state);
                let response_json = serde_json::to_string(&response).unwrap();
                println!("json is: {}", response_json);
                let data_respond = response_json.as_bytes();
                stream.write(data_respond).unwrap();
                //thread::sleep(time::Duration::from_millis(500));
                //Ok(());
            }
        } else {
            println!("no mpv socket");
            let response = PlaybackUpdateMessage::new(0, 0);
            let response_json = serde_json::to_string(&response).unwrap();
            println!("json is: {}", response_json);
            let data_respond = response_json.as_bytes();
            stream.write(data_respond).unwrap();
            //Ok(());
        }
    }
}

//fn upd_volume() -> VolumeUpdateMessage {
//
//}

pub fn update_state2() -> PlaybackUpdateMessage {
    //let mut message: PlaybackUpdateMessage;
    //thread::sleep(time::Duration::from_millis(2000));
    //let message = update_playback();
    //let response_json = serde_json::to_string(&message).unwrap();
    //println!("json is: {}", response_json);
    //let data_respond = response_json.as_bytes();
    //stream.write(data_respond).unwrap();
    //thread::sleep(time::Duration::from_millis(100));
    return update_playback();
}

fn update_playback() -> PlaybackUpdateMessage {
    let mpv = Mpv::connect(MPVSOCKET);
    match mpv {
        Result::Ok(mpv) => {
            //let idle: bool = mpv.get_property("file-loaded").unwrap();
            //if idle == true {
            //    println!("returning idle");
            //    return PlaybackUpdateMessage::new(0, 0);
            //} else {
            let playback_time: f64 = mpv.get_property("playback-time").unwrap();
            //let paused: bool = mpv.get_property("pause").unwrap();
            let time_state = playback_time as u64;
            let paused_bool: bool = mpv.get_property("pause").unwrap();
            //let paused_state = PlayState::new();
            let mut paused_state: u8;
            if paused_bool == true {
                paused_state = 2;
            } else {
                paused_state = 1;
            }
            println!("returning non idle");
            return PlaybackUpdateMessage::new(time_state, paused_state);
            //}
        }
        Result::Err(_) => {
            println!("returning error");
            return PlaybackUpdateMessage::new(0, 0);
        }
    }
}

fn checksock() -> bool {
    let path = Path::new(MPVSOCKET);
    return path.exists();
}
