use mem::str::String32;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct AudioDevice {
    pub id: String32,
    pub name: String32,
    pub io_size: (usize, usize),
}

impl AudioDevice {
    //pub fn from_aplay_str(str: String32) -> Option<AudioDevice> {
    //    //card 1: Headphones [bcm2835 Headphones], device 0: bcm2835 Headphones [bcm2835 Headphones]
    //    let card_idx = &str[str.find("card")? + 5..str.find(':')?];
    //    let _device_idx = &str[(str.find("device")? + 7)..(str[8..].find(':')? + 8)];
    //    let id = format!("hw:{card_idx}");
    //    Some(Self {
    //        id,
    //        io_size: (0, 0),
    //        name: str,
    //    })
    //}
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct JACKStatus {
    pub available_devices: [Option<AudioDevice>; 8],
    pub io_size: (usize, usize),
    pub buffer_size: usize,
    pub sample_rate: usize,
    pub frame_size: usize,
    pub connections: [u32; 32],
    pub client_name: String32,
    pub output_name: String32,
    pub running: bool,
}
