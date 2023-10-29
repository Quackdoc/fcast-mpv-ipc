use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug, PartialEq)]
pub enum Opcode {
    None = 0,
    Play = 1,
    Pause = 2,
    Resume = 3,
    Stop = 4,
    Seek = 5,
    PlaybackUpdate = 6,
    VolumeUpdate = 7,
    SetVolume = 8
}

impl Opcode {
    pub fn from_u8(value: u8) -> Opcode {
        match value {
            0 => Opcode::None,
            1 => Opcode::Play,
            2 => Opcode::Pause,
            3 => Opcode::Resume,
            4 => Opcode::Stop,
            5 => Opcode::Seek,
            6 => Opcode::PlaybackUpdate,
            7 => Opcode::VolumeUpdate,
            8 => Opcode::SetVolume,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum PlayState {
    Idle = 0,
    Playing = 1,
    Paused = 2
}

impl PlayState {
    pub fn new(value: u8) -> PlayState {
        match value {
            0 => PlayState::Idle,
            1 => PlayState::Playing,
            2 => PlayState::Paused,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

//HEADER
#[derive(Deserialize, Debug)]
pub struct HEADER {
    pub size: u32,
    pub opcode: Opcode,
}

impl HEADER {
    pub fn new(size: u32, opcode: Opcode,) -> Self {
        Self { size, opcode }
    }
}

//BODIES
#[derive(Deserialize, Debug)]
pub struct PlayMessage {
    pub container: String,
    pub url: Option<String>,
    pub content: Option<String>,
    pub time: Option<u64>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct SeekMessage {
    pub time: u64,
}


#[derive(Deserialize, Debug, Serialize)]
pub struct PlaybackUpdateMessage {
    pub time: u64,
    pub state: u8 //0 = None, 1 = Playing, 2 = Paused
}

impl PlaybackUpdateMessage {
    pub fn new(time: u64, state: u8, ) -> Self {
        Self { time, state }
    }
}
#[derive(Deserialize, Debug, Serialize)]
pub struct VolumeUpdateMessage {
    pub volume: f64 //(0-1)
}

impl VolumeUpdateMessage {
    pub fn new(volume: f64 ) -> Self {
        Self { volume }
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct SetVolumeMessage {
    pub volume: f64,
}
