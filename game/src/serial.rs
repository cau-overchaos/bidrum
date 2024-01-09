use std::{
    sync::{mpsc, Arc, RwLock},
    thread::sleep,
    time::Duration,
};

use serialport::SerialPort;

// Korean looks more intutitive than english...
#[derive(Debug)]
pub enum DrumPane {
    채편,
    북편,
}

#[derive(Debug)]
pub(crate) struct ControllerState {
    pub 궁채: Option<DrumPane>,
    pub 북채: Option<DrumPane>,
}

pub(crate) fn empty_controller_state() -> ControllerState {
    return ControllerState {
        궁채: None,
        북채: None,
    };
}

pub(crate) fn read_serial_loop(mut port: Box<dyn SerialPort>, lock: Arc<RwLock<ControllerState>>) {
    loop {
        loop {
            let available_bytes: u32 = port.bytes_to_read().expect("Failed to read buffer size");
            if available_bytes > 0 {
                break;
            }
            sleep(Duration::from_millis(10));
        }
        let mut message: Vec<u8> = vec![0; 1];

        port.read_exact(message.as_mut_slice())
            .expect("Controller reading failure!");

        let state = ControllerState {
            궁채: if message[0] & 1 != 0 {
                Some(DrumPane::채편)
            } else if message[0] & 2 != 0 {
                Some(DrumPane::북편)
            } else {
                None
            },
            북채: if message[0] & 4 != 0 {
                Some(DrumPane::채편)
            } else if message[0] & 8 != 0 {
                Some(DrumPane::북편)
            } else {
                None
            },
        };

        let result = lock.write();
        match result {
            Ok(mut guard) => {
                guard.궁채 = state.궁채;
                guard.북채 = state.북채;
            }
            Err(_) => {}
        }
    }
}
