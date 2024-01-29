use std::{
    sync::{atomic::AtomicU8, Arc},
    thread::sleep,
    time::Duration,
};

use bidrum_data_struct_lib::janggu::JangguFace;
use serialport::SerialPort;

use bidrum_data_struct_lib::janggu::JangguInputState;

pub(crate) fn parse_janggu_bits(bits: u8) -> JangguInputState {
    JangguInputState {
        궁채: if bits & 1 != 0 {
            Some(JangguFace::채편)
        } else if bits & 2 != 0 {
            Some(JangguFace::북편)
        } else {
            None
        },
        북채: if bits & 4 != 0 {
            Some(JangguFace::채편)
        } else if bits & 8 != 0 {
            Some(JangguFace::북편)
        } else {
            None
        },
    }
}

/// Read serial inputs from port and emulates key inputs
pub(crate) fn read_serial_loop(mut port: Box<dyn SerialPort>, bits_data: Arc<AtomicU8>) {
    loop {
        loop {
            let available_bytes: u32 = port.bytes_to_read().expect("Failed to read buffer size");
            if available_bytes > 0 {
                break;
            }
            sleep(Duration::from_millis(10));
        }
        let mut message: [u8; 1] = [0];

        port.read_exact(message.as_mut_slice())
            .expect("Controller reading failure!");

        bits_data.store(message[0], std::sync::atomic::Ordering::Relaxed);
    }
}
