use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug, PartialEq)]
pub struct Mpv {
    pub socket: String,
    pub args: String, 
}
#[derive(Deserialize, Debug, PartialEq)]
Pub struct Fcast {
    pub ipaddr: String,
    pub port: i32,
}