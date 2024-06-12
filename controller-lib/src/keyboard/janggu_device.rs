use std::{
    sync::{
        atomic::{AtomicBool, AtomicU8, Ordering},
        Arc, RwLock,
    },
    thread,
};

use bidrum_data_struct_lib::janggu::{JangguFace, JangguInputState};
use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::JangguDevice;

pub struct KeyboardJangguDevice {
    stopping: Arc<AtomicBool>,
    // Using RwLock<JangguInputState> is too slow
    state: Arc<AtomicU8>,
}

impl KeyboardJangguDevice {
    pub fn new() -> KeyboardJangguDevice {
        let stopping = Arc::new(AtomicBool::new(false));
        let state = Arc::new(AtomicU8::new(0));

        {
            let stopping = stopping.clone();
            let state = state.clone();

            thread::spawn(move || {
                let mut device_states = DeviceState::new();
                loop {
                    if stopping.load(Ordering::Relaxed) {
                        break;
                    }

                    state.store(keyboard_to_bits(&mut device_states), Ordering::Relaxed);
                }
            });
        }

        KeyboardJangguDevice {
            state: state,
            stopping: stopping,
        }
    }
}

impl Drop for KeyboardJangguDevice {
    fn drop(&mut self) {
        self.stopping.store(true, Ordering::Relaxed);
    }
}

impl JangguDevice for KeyboardJangguDevice {
    fn read_janggu_input_state(&self) -> JangguInputState {
        bits_to_janggu_input_state(self.state.load(Ordering::Relaxed))
    }
}

fn bits_to_janggu_input_state(bits: u8) -> JangguInputState {
    JangguInputState {
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
    }
}

fn keyboard_to_bits(device_states: &mut DeviceState) -> u8 {
    let keys = device_states.get_keys();
    let mut bits = 0;
    if keys.contains(&Keycode::D) {
        bits |= 1;
    } else if keys.contains(&Keycode::F) {
        bits |= 2;
    }
    if keys.contains(&Keycode::J) {
        bits |= 4;
    } else if keys.contains(&Keycode::K) {
        bits |= 8;
    }

    return bits;
}
