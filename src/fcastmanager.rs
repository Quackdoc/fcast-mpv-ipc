//mod models;

use core::panic;
use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};
use std::time::Duration;
use std::{time, thread};
use serde_json::Value;

use crate::models::{HEADER,Opcode, VolumeUpdateMessage, PlaybackUpdateMessage};
use crate::command::{play, seek, setvol, pause, resume, stop};
use crate::state::{update_state2, check_for_mpv};

//const LENGTH_BYTES: usize = 4;
const MAXIMUM_PACKET_LENGTH: usize = 35000;

pub fn main_handler(stream: TcpStream) {
    listen2(stream);
    
}

fn listen2(mut stream: TcpStream) {
    let mut data = [0 as u8; MAXIMUM_PACKET_LENGTH];
    let mut combined_data: Vec<u8> = vec![];
    stream.set_read_timeout(Some(Duration::new(0, 250))).unwrap();
    //stream.set_write_timeout(Some(Duration::new(0, 250))).unwrap();
    //stream.set_nonblocking(false).unwrap();
    loop {
        //println!("loop start");
        //println!("\n");
        
        //let dur = time::Duration::from_millis(250);
        let input = stream.read(&mut data);
            match input {
            Ok (size) => {
            if size == 4 {
                //println!("usize is 4");
                combined_data.extend_from_slice(&data[0..4]);
            } else if size == 0 {
                println!("usize is 0, empty buffer");
            } else {
                //println!("usize is at least {}", size);
                combined_data.extend_from_slice(&data[0..size]);
                println!("combined_data length in main loop is: {}", combined_data.len() );
                //println!("data usize is: {}", size);
                //println!("data: {:?}", &combined_data[0..size + 4]);
                let header_info = read_header(&combined_data[0..5]);
                //println!("opcode: {:?}", &header_info.opcode);
                
                if header_info.size == 0 {
                    panic!("size should never be zero");
                } else {
                    println!("header info size: {:?}", &header_info.size);
                }
                println!("start match");
                match header_info.opcode {
                    Opcode::None => println!("No Body, Opcode is {:?}", header_info.opcode),
                    Opcode::Play => {
                        thread::spawn(move|| {
                            play(read_body(&combined_data, header_info.size))});  
                        },
                    Opcode::Pause => {
                            thread::spawn(move|| {pause()});
                        },
                    Opcode::Resume => {
                            thread::spawn(move|| {resume()});
                        },
                    Opcode::Stop => {
                            thread::spawn(move|| {stop()});
                            },
                    Opcode::Seek => {
                            thread::spawn(move|| { seek(read_body(&combined_data, header_info.size))});
                            },
                    Opcode::PlaybackUpdate => panic!("Incomming stream should never be {:?}", header_info.opcode),
                    Opcode::VolumeUpdate => panic!("Incomming stream should never be {:?}", header_info.opcode),
                    Opcode::SetVolume => {
                            thread::spawn(move|| {setvol(read_body(&combined_data, header_info.size))});
                            },
                }
                //println!("finished match");
                stream.flush().unwrap();
                combined_data = vec![];
                println!("combined data");
            }
            },
            Err (_) => {
                println!("no data to read");
            },
        }

        if check_for_mpv() {
            let data_respond = craft_response();
            //println!("sending json data: {:?}", data_respond);
            let status = stream.write(&data_respond);
            match status {
                Ok(size) => println!("Write succsess usize is {}", size),
                Err(err) => println!("error is {:?}", err),
            }
            stream.flush().unwrap();
            thread::sleep(time::Duration::from_secs(1));
        } else {
            println!("not replying");
        }
        thread::sleep(time::Duration::from_millis(1000));
        
    }

}

fn craft_response() -> Vec<u8> {
    //println!("crafting response");
    let response = update_state2();
    //let response_json = serde_json::to_string(&response).unwrap();
    let response_txt = format!("{{ time: {}, state: {} }}", response.time, response.state );
    //let response_txt = "{ time: 1, state: 1 }";
    println!("{{ time: {}, state: {} }}", response.time, response.state );
    
    let msg_respond = response_txt.as_bytes();
    let data_respond = craft_data(msg_respond, Opcode::PlaybackUpdate);
    return data_respond;
}

fn craft_data(message: &[u8], opcode: Opcode) -> Vec<u8> {

    let message_size = message.len() as u32 + 1;
    let op: u8 = opcode as u8;
    let size_bytes = message_size.to_ne_bytes();
    let mut header_bytes: Vec<u8> = vec![];
    header_bytes.extend_from_slice(&size_bytes);
    header_bytes.push(op);
    header_bytes.extend_from_slice(&message);
    //println!("header bytes are {:?}", header_bytes);
    return header_bytes;
}

fn read_header(data: &[u8]) -> HEADER {

    let header_size_bytes = &data[0..4];
    let header_size = u32::from_ne_bytes(header_size_bytes.try_into().unwrap());
    let header_op = data[4];
    println!("Opcode is: {:?}", Opcode::from_u8(header_op));
    return HEADER::new(header_size, Opcode::from_u8(header_op));
}

fn read_body(data: &[u8], size: u32 ) -> Value {
    //println!("recieved size is: {}", size);
    let body = &data[5..size as usize + 4]; //ofset 5 bytes - 1 as per spec
    let json_data_string = std::str::from_utf8(body).unwrap();
    //println!("BODY: {:?}", &json_data_string);
    let json_data: Value = serde_json::from_str(json_data_string).unwrap();
    //println!("Ready the incomming body");
    println!("Incomming Data: {:?}", json_data);
    return json_data;
}