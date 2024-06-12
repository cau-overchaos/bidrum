use std::{
    sync::{
        atomic::{AtomicBool, AtomicU32, AtomicU8, Ordering},
        Arc, RwLock,
    },
    thread,
};

use bidrum_controller_lib::{keyboard, serial, CoinInputDevice, JangguDevice};
use bidrum_data_struct_lib::janggu::{JangguFace, JangguInputState};

/// Wrapper of Coin/Janggu controller
/// to avoid cumbersome ownership/borrow/lifetime problems
pub struct ControllerWrapper {
    janggu_state: Arc<AtomicU8>,
    coins: Arc<AtomicU32>,
    coins_to_consume: Arc<AtomicU32>,
    stopping: Arc<AtomicBool>,
}

fn janggu_state_to_u8(state: JangguInputState) -> u8 {
    let mut result: u8 = 0;
    result |= match state.궁채 {
        Some(JangguFace::궁편) => 1,
        Some(JangguFace::열편) => 2,
        _ => 0,
    };
    result |= match state.열채 {
        Some(JangguFace::궁편) => 4,
        Some(JangguFace::열편) => 8,
        _ => 0,
    };
    return result;
}

fn u8_to_janggu_state(bits: u8) -> JangguInputState {
    JangguInputState {
        궁채: if (bits & 1) != 0 {
            Some(JangguFace::궁편)
        } else if (bits & 2) != 0 {
            Some(JangguFace::열편)
        } else {
            None
        },
        열채: if (bits & 4) != 0 {
            Some(JangguFace::궁편)
        } else if (bits & 8) != 0 {
            Some(JangguFace::열편)
        } else {
            None
        },
    }
}

impl Drop for ControllerWrapper {
    fn drop(&mut self) {
        self.stopping.store(true, Ordering::Relaxed);
    }
}

impl ControllerWrapper {
    pub fn read_janggu_state(&self) -> JangguInputState {
        u8_to_janggu_state(self.janggu_state.load(Ordering::Relaxed))
    }
    pub fn get_coins(&self) -> u32 {
        self.coins.load(Ordering::Relaxed)
    }
    pub fn consume_coins(&mut self, coins: u32) {
        self.coins_to_consume.fetch_add(coins, Ordering::Relaxed);
    }
    pub fn keyboard() -> ControllerWrapper {
        let coins = Arc::new(AtomicU32::new(0));
        let coins_to_consume = Arc::new(AtomicU32::new(0));
        let stopping = Arc::new(AtomicBool::new(false));
        let janggu_state = Arc::new(AtomicU8::new(0));
        {
            let coins = coins.clone();
            let coins_to_consume = coins_to_consume.clone();
            let stopping = stopping.clone();
            let janggu_state = janggu_state.clone();

            thread::spawn(move || {
                let mut coin_device = keyboard::coin_device::KeyboardCoinDevice::new();
                let janggu_device = keyboard::janggu_device::KeyboardJangguDevice::new();

                loop {
                    if stopping.load(Ordering::Relaxed) {
                        break;
                    }

                    let consume = coins_to_consume.load(Ordering::Relaxed);
                    if consume > 0 {
                        coin_device.consume_coins(consume);
                        coins_to_consume.fetch_sub(consume, Ordering::Relaxed);
                    }

                    coins.store(coin_device.get_unconsumed_coins(), Ordering::Relaxed);
                    janggu_state.store(
                        janggu_state_to_u8(janggu_device.read_janggu_input_state()),
                        Ordering::Relaxed,
                    );
                }
            });
        }

        ControllerWrapper {
            janggu_state: janggu_state,
            coins: coins,
            coins_to_consume: coins_to_consume,
            stopping: stopping,
        }
    }

    pub fn serial(controller_port: String) -> ControllerWrapper {
        let coins = Arc::new(AtomicU32::new(0));
        let coins_to_consume = Arc::new(AtomicU32::new(0));
        let stopping = Arc::new(AtomicBool::new(false));
        let janggu_state = Arc::new(AtomicU8::new(0));
        {
            let coins = coins.clone();
            let coins_to_consume = coins_to_consume.clone();
            let stopping = stopping.clone();
            let janggu_state = janggu_state.clone();

            thread::spawn(move || {
                let (janggu_device, mut coin_device) = serial::new(controller_port);

                loop {
                    if stopping.load(Ordering::Relaxed) {
                        break;
                    }

                    let consume = coins_to_consume.load(Ordering::Relaxed);
                    if consume > 0 {
                        coin_device.consume_coins(consume);
                        coins_to_consume.fetch_sub(consume, Ordering::Relaxed);
                    }

                    coins.store(coin_device.get_unconsumed_coins(), Ordering::Relaxed);
                    janggu_state.store(
                        janggu_state_to_u8(janggu_device.read_janggu_input_state()),
                        Ordering::Relaxed,
                    );
                }
            });
        }

        ControllerWrapper {
            janggu_state: janggu_state,
            coins: coins,
            coins_to_consume: coins_to_consume,
            stopping: stopping,
        }
    }
}
