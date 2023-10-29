//mod models;

use core::panic;
use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};
use std::time::Duration;
use std::{time, thread};
use serde_json::Value;

use crate::models::{HEADER,Opcode, VolumeUpdateMessage, PlaybackUpdateMessage};
use crate::command::{play, seek, setvol, pause, resume, stop};
use crate::state::update_state2;

//const LENGTH_BYTES: usize = 4;
const MAXIMUM_PACKET_LENGTH: usize = 35000;

pub fn main_handler(stream: TcpStream) {
    listen2(stream);
    
}

fn listen2(mut stream: TcpStream) {
    let mut data = [0 as u8; MAXIMUM_PACKET_LENGTH];
    let mut combined_data: Vec<u8> = vec![];

    loop {
        println!("loop start");
        println!("\n");
        
        //let dur = time::Duration::from_millis(250);
        stream.set_read_timeout(Some(Duration::new(1, 0))).unwrap();
        let input = stream.read(&mut data);
            match input {
            Ok (size) => {
            if size == 4 {
                println!("usize is 4");
                combined_data.extend_from_slice(&data[0..4]);
            } else if size == 0 {
                println!("usize is 0, empty buffer");
            } else {
                println!("usize is at least {}", size);
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
                println!("finished match");
                stream.flush().unwrap();
                combined_data = vec![];
                println!("combined data");
            }
            },
            Err (_) => {
                println!("no data to read");
            },
        }

        println!("crafting response");
        let response = update_state2();
        //let response_json = serde_json::to_string(&response).unwrap();
        let response_txt = format!("{{ time: {}, state: {} }}", response.time, response.state );
        //let response_txt = "{ time: {}, state: {} }";
        //println!("\{ time: {}, state: {} \}", response.time, response.state );
        println!("sending json data: {}", response_txt);
        let data_respond = response_txt.as_bytes();
        stream.write(data_respond).unwrap();
        thread::sleep(time::Duration::from_millis(250));
        println!("send the data");
    }

}

//fn listener(mut stream: TcpStream) {
//    let mut data = [0 as u8; MAXIMUM_PACKET_LENGTH]; // using 32k buffer
//    //let mut buf = vec![];
//    //let mut header = [0 as u8; 5];
//    //let mut pull_size: u8 = 0;
//    let mut combined_data: Vec<u8> = vec![];
//    //TODO: grayjay first packet is the length header, second packet has opcode + body
//    while match stream.read(&mut data) {
//        Ok(size) => {
//            println!("\n");
//            if size == 4 {
//                //println!("data size should be: {:?}", &data[0..4]);
//                //let bytes = &data[0..4];
//                //pull_size = u32::from_ne_bytes(bytes.try_into().unwrap()) as u8;
//                combined_data.extend_from_slice(&data[0..4]);
//                true
//            } else {
//                combined_data.extend_from_slice(&data[0..size]);
//                //println!("data usize is: {}", size);
//                //println!("data: {:?}", &combined_data[0..size + 4]);
//                let header_info = read_header(&combined_data[0..5]);
//                //println!("opcode: {:?}", &header_info.opcode);
//                
//                if header_info.size == 0 {
//                    panic!("size should never be zero");
//                } else {
//                    println!("header info size: {:?}", &header_info.size);
//                }
//                match header_info.opcode {
//                    Opcode::None => println!("No Body, Opcode is {:?}", header_info.opcode),
//                    Opcode::Play => {
//                        thread::spawn(move|| {
//                            play(read_body(&combined_data, header_info.size))});  
//                        },
//                    Opcode::Pause => {
//                            thread::spawn(move|| {pause()});
//                        },
//                    Opcode::Resume => {
//                            thread::spawn(move|| {resume()});
//                        },
//                    Opcode::Stop => {
//                            thread::spawn(move|| {stop()});
//                            },
//                    Opcode::Seek => {
//                            thread::spawn(move|| { seek(read_body(&combined_data, header_info.size))});
//                            },
//                    Opcode::PlaybackUpdate => panic!("Incomming stream should never be {:?}", header_info.opcode),
//                    Opcode::VolumeUpdate => panic!("Incomming stream should never be {:?}", header_info.opcode),
//                    Opcode::SetVolume => {
//                            thread::spawn(move|| {setvol(read_body(&combined_data, header_info.size))});
//                            },
//                }
//                let response = update_state2();
//                let response_json = serde_json::to_string(&response).unwrap();
//                //println!("json is: {}", response_json);
//                let data_respond = response_json.as_bytes();
//                stream.write(data_respond).unwrap();
//                thread::sleep(time::Duration::from_millis(250));
//                //println!("\n");
//                //data.clear();
//                stream.flush().unwrap();
//                //combined_data.clear();
//                combined_data = vec![];
//                false
//            }
//        },
//        Err(_) => {
//             println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
//             stream.shutdown(Shutdown::Both).unwrap();
//             false
//        }
//    } {}
//}

fn read_header(data: &[u8]) -> HEADER {

    let header_size_bytes = &data[0..4];
    let header_size = u32::from_ne_bytes(header_size_bytes.try_into().unwrap());
    let header_op = data[4];

    println!("opcode is {}", &header_op);
    println!("header_size is {}", &header_size);
    
    return HEADER::new(header_size, Opcode::from_u8(header_op));
}

fn read_body(data: &[u8], size: u32 ) -> Value {
    println!("recieved size is: {}", size);
    let body = &data[5..size as usize + 4]; //ofset 5 bytes - 1 as per spec
    let json_data_string = std::str::from_utf8(body).unwrap();
    //println!("BODY: {:?}", &json_data_string);
    let json_data: Value = serde_json::from_str(json_data_string).unwrap();
    println!("Ready the incomming body");
    println!("JSON: {:?}", json_data);
    return json_data;
}