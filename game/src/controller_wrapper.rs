use std::{
    sync::{
        atomic::{AtomicBool, AtomicU32, Ordering},
        Arc, RwLock,
    },
    thread,
};

use bidrum_controller_lib::{keyboard, serial, CoinInputDevice, JangguDevice};
use bidrum_data_struct_lib::janggu::JangguInputState;

/// Wrapper of Coin/Janggu controller
/// to avoid cumbersome ownership/borrow/lifetime problems
pub struct ControllerWrapper {
    janggu_state: Arc<RwLock<JangguInputState>>,
    coins: Arc<AtomicU32>,
    coins_to_consume: Arc<AtomicU32>,
    stopping: Arc<AtomicBool>,
}

impl Drop for ControllerWrapper {
    fn drop(&mut self) {
        self.stopping.store(true, Ordering::Relaxed);
    }
}

impl ControllerWrapper {
    pub fn read_janggu_state(&self) -> JangguInputState {
        *self.janggu_state.read().expect("Failed to get lock")
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
        let janggu_state = Arc::new(RwLock::new(JangguInputState {
            궁채: None,
            열채: None,
        }));
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
                    let mut janggu_state = janggu_state.write().expect("Failed to get lock");
                    *janggu_state = janggu_device.read_janggu_input_state();
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
        let janggu_state = Arc::new(RwLock::new(JangguInputState {
            궁채: None,
            열채: None,
        }));
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
                    let mut janggu_state = janggu_state.write().expect("Failed to get lock");
                    *janggu_state = janggu_device.read_janggu_input_state();
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
