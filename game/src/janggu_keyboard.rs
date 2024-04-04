use std::sync::{atomic::AtomicU8, Arc};

use device_query::{DeviceQuery, DeviceState, Keycode};

/// Read keyboard inputs and save janggu input bits
pub(crate) fn read_janggu_key_loop(bits_data: Arc<AtomicU8>) {
    loop {
        let mut bits: u8 = 0;
        let device_states = DeviceState::new();
        let keys = device_states.get_keys();
        if keys.contains(&Keycode::D) {
            bits |= 1;
        }
        if keys.contains(&Keycode::F) {
            bits |= 2;
        }
        if keys.contains(&Keycode::J) {
            bits |= 4;
        }
        if keys.contains(&Keycode::K) {
            bits |= 8;
        }

        bits_data.store(bits, std::sync::atomic::Ordering::Relaxed);
    }
}
