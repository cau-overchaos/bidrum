use std::{
    sync::{
        atomic::{AtomicBool, AtomicU32, Ordering},
        Arc,
    },
    time::Duration,
};

use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::CoinInputDevice;

/// United bidrum controller of Janggu and Coin/Bill acceptor
pub struct KeyboardCoinDevice {
    unconsumed_coins: Arc<AtomicU32>,
    stopping: Arc<AtomicBool>,
}

impl Drop for KeyboardCoinDevice {
    fn drop(&mut self) {
        self.stopping.store(true, Ordering::Relaxed);
    }
}

impl CoinInputDevice for KeyboardCoinDevice {
    fn get_unconsumed_coins(&self) -> u32 {
        self.unconsumed_coins.load(Ordering::Relaxed)
    }

    fn consume_coins(&mut self, coins: u32) {
        self.unconsumed_coins.fetch_sub(coins, Ordering::Relaxed);
    }
}

impl KeyboardCoinDevice {
    pub fn new() -> KeyboardCoinDevice {
        let unconsumed_coins = Arc::new(AtomicU32::new(0));
        let stopping = Arc::new(AtomicBool::new(false));

        {
            let unconsumed_coins = unconsumed_coins.clone();

            let _stopping = stopping.clone();

            std::thread::spawn(move || {
                let mut pressed = false;
                let device_state = DeviceState::new();
                loop {
                    let new_pressed = device_state.get_keys().contains(&Keycode::C);
                    if new_pressed && !pressed {
                        // increase one on keydown
                        unconsumed_coins.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    }

                    pressed = new_pressed;
                    std::thread::sleep(Duration::from_millis(10));
                }
            });
        }

        KeyboardCoinDevice {
            unconsumed_coins: unconsumed_coins,
            stopping: stopping,
        }
    }
}

fn has_coin_bit(bits: u8) -> bool {
    bits & 16 != 0
}
