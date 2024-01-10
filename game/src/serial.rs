use std::{
    sync::{mpsc, Arc, RwLock},
    thread::sleep,
    time::Duration,
};

use enigo::{Enigo, KeyboardControllable, Key};
use serialport::SerialPort;

// Korean looks more intutitive than english...
#[derive(Debug, Clone, Copy)]
pub enum DrumPane {
    채편,
    북편,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ControllerState {
    pub 궁채: Option<DrumPane>,
    pub 북채: Option<DrumPane>,
}

pub(crate) fn keys_to_state_struct(key1: bool, key2: bool, key3: bool, key4: bool) -> ControllerState {
    ControllerState {
        궁채: if key1 {
            Some(DrumPane::채편)
        } else if key2 {
            Some(DrumPane::북편)
        } else {
            None
        },
        북채: if key3 {
            Some(DrumPane::채편)
        } else if key4 {
            Some(DrumPane::북편)
        } else {
            None
        },
    }
}

fn state_struct_to_keys(state: ControllerState) -> (bool, bool, bool, bool) {
    return (
        match state.궁채 {
            Some(pane) => matches!(pane, DrumPane::채편),
            None => false
        },
        match state.궁채 {
            Some(pane) => matches!(pane, DrumPane::북편),
            None => false
        },
        match state.북채 {
            Some(pane) => matches!(pane, DrumPane::채편),
            None => false
        },
        match state.북채 {
            Some(pane) => matches!(pane, DrumPane::북편),
            None => false
        }
    )
}

pub(crate) fn read_serial_loop(mut port: Box<dyn SerialPort>) {
    let mut previous_state = ControllerState {
        궁채: None,
        북채: None
    };
    let mut enigo = Enigo::new();

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

        let current_state = ControllerState {
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

        let previous_keys = state_struct_to_keys(previous_state);
        let current_keys = state_struct_to_keys(current_state);
        let keys = vec!['u', 'i', 'o', 'p'];

        for i in 0..4 {
            let previous_key = match i {
                0 => previous_keys.0,
                1 => previous_keys.1,
                2 => previous_keys.2,
                3 => previous_keys.3,
                _ => panic!("Unexpected loop index value")
            };
            let current_key = match i {
                0 => current_keys.0,
                1 => current_keys.1,
                2 => current_keys.2,
                3 => current_keys.3,
                _ => panic!("Unexpected loop index value")
            };

            if previous_key != current_key {
                if previous_key {
                    enigo.key_up(Key::Layout(keys[i]));
                } else {
                    enigo.key_down(Key::Layout(keys[i]));
                }
            }
        }

        previous_state = current_state;
    }
}
