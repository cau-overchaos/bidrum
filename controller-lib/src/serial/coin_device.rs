use std::{
    sync::{
        atomic::{AtomicBool, AtomicU32, Ordering},
        Arc,
    },
    thread::{self},
};

use crate::CoinInputDevice;

use super::serial_reader::BidrumSerialReader;

/// United bidrum controller of Janggu and Coin/Bill acceptor
pub struct SerialCoinDevice {
    serial_reader: BidrumSerialReader,
    unconsumed_coins: Arc<AtomicU32>,
    stopping: Arc<AtomicBool>,
}

impl Drop for SerialCoinDevice {
    fn drop(&mut self) {
        self.stopping.store(true, Ordering::Relaxed);
    }
}

impl CoinInputDevice for SerialCoinDevice {
    fn get_unconsumed_coins(&self) -> u32 {
        self.unconsumed_coins.load(Ordering::Relaxed)
    }

    fn consume_coins(&mut self, coins: u32) {
        self.unconsumed_coins.fetch_sub(coins, Ordering::Relaxed);
    }
}

impl SerialCoinDevice {
    pub(super) fn new(serial_reader: BidrumSerialReader) -> SerialCoinDevice {
        let unconsumed_coins = Arc::new(AtomicU32::new(0));
        let stopping = Arc::new(AtomicBool::new(false));

        {
            let unconsumed_coins = unconsumed_coins.clone();
            let bits = serial_reader.bits.clone();
            let stopping = stopping.clone();
            thread::spawn(move || {
                let mut prev = false;
                loop {
                    if stopping.load(Ordering::Relaxed) {
                        break;
                    }

                    let current = has_coin_bit(bits.load(Ordering::Relaxed));
                    if !prev && current {
                        unconsumed_coins.fetch_add(1, Ordering::Relaxed);
                    }

                    prev = current;
                }
            });
        }

        SerialCoinDevice {
            serial_reader: serial_reader,
            unconsumed_coins: unconsumed_coins,
            stopping: stopping,
        }
    }
}

fn has_coin_bit(bits: u8) -> bool {
    bits & 16 != 0
}
