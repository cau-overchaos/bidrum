use std::{thread::sleep, time::Duration};

use enigo::{Enigo, Key, KeyboardControllable};
use serialport::SerialPort;

use crate::janggu::{DrumPane, JangguState};

/// Convert key inputs to janggu state struct object
pub(crate) fn keys_to_state_struct(key1: bool, key2: bool, key3: bool, key4: bool) -> JangguState {
    JangguState {
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

/// Convert janggu state struct object to whether keys are pressed
fn state_struct_to_keys(state: JangguState) -> (bool, bool, bool, bool) {
    return (
        match state.궁채 {
            Some(pane) => matches!(pane, DrumPane::채편),
            None => false,
        },
        match state.궁채 {
            Some(pane) => matches!(pane, DrumPane::북편),
            None => false,
        },
        match state.북채 {
            Some(pane) => matches!(pane, DrumPane::채편),
            None => false,
        },
        match state.북채 {
            Some(pane) => matches!(pane, DrumPane::북편),
            None => false,
        },
    );
}

/// Read serial inputs from port and emulates key inputs
pub(crate) fn read_serial_loop(mut port: Box<dyn SerialPort>) {
    let mut previous_state = JangguState {
        궁채: None,
        북채: None,
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

        // Parse serial input with bitmask
        let current_state = JangguState {
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
                _ => panic!("Unexpected loop index value"),
            };
            let current_key = match i {
                0 => current_keys.0,
                1 => current_keys.1,
                2 => current_keys.2,
                3 => current_keys.3,
                _ => panic!("Unexpected loop index value"),
            };

            // Emulate keyup, keypress, keydown correctly
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
