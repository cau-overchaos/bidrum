use std::sync::atomic::Ordering;

use bidrum_data_struct_lib::janggu::{JangguFace, JangguInputState};

use crate::JangguDevice;

use super::serial_reader::BidrumSerialReader;

pub struct SerialJangguDevice {
    serial_reader: BidrumSerialReader,
}

impl SerialJangguDevice {
    pub(super) fn new(serial_reader: BidrumSerialReader) -> SerialJangguDevice {
        SerialJangguDevice {
            serial_reader: serial_reader,
        }
    }
}

impl JangguDevice for SerialJangguDevice {
    fn read_janggu_input_state(&self) -> JangguInputState {
        return parse_janggu_bits(self.serial_reader.bits.load(Ordering::Relaxed));
    }
}

fn parse_janggu_bits(bits: u8) -> JangguInputState {
    return JangguInputState {
        궁채: if bits & 1 != 0 {
            Some(JangguFace::궁편)
        } else if bits & 2 != 0 {
            Some(JangguFace::열편)
        } else {
            None
        },
        열채: if bits & 4 != 0 {
            Some(JangguFace::궁편)
        } else if bits & 8 != 0 {
            Some(JangguFace::열편)
        } else {
            None
        },
    };
}
