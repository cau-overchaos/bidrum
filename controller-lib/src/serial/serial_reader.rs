use std::{
    sync::{
        atomic::{AtomicBool, AtomicU32, AtomicU8, Ordering},
        Arc,
    },
    thread::{self, sleep},
    time::Duration,
};

use serialport::SerialPort;

/// United bidrum controller of Janggu and Coin/Bill acceptor
pub(super) struct BidrumSerialReader {
    stopping: Arc<AtomicBool>,
    pub(super) bits: Arc<AtomicU8>,
    counter: Arc<AtomicU32>,
}

impl Clone for BidrumSerialReader {
    fn clone(&self) -> Self {
        self.counter.fetch_add(1, Ordering::Relaxed);

        Self {
            stopping: self.stopping.clone(),
            bits: self.bits.clone(),
            counter: self.counter.clone(),
        }
    }
}

impl Drop for BidrumSerialReader {
    fn drop(&mut self) {
        self.counter.fetch_sub(1, Ordering::Relaxed);

        if self.counter.load(Ordering::Relaxed) == 0 {
            self.stopping.store(true, Ordering::Relaxed);
        }
    }
}

impl BidrumSerialReader {
    pub(super) fn new(controller_port: String) -> BidrumSerialReader {
        println!("Openning serial port {}", controller_port);

        let mut port = serialport::new(controller_port, 9600)
            .timeout(Duration::from_millis(20))
            .open()
            .expect("Failed to open port");

        println!("Waiting 3 seconds (Arduino needs time for serial initialization)");
        sleep(Duration::from_millis(3000));
        println!("Waited 3 seconds!");

        let stopping = Arc::new(AtomicBool::new(false));
        let bits = Arc::new(AtomicU8::new(0));
        let counter = Arc::new(AtomicU32::new(1));
        {
            let stopping = stopping.clone();
            let bits = bits.clone();

            thread::spawn(move || loop {
                if stopping.load(Ordering::Relaxed) {
                    break;
                }
                read_serial(&mut port, &bits);
            });
        }

        BidrumSerialReader {
            stopping: stopping,
            counter: counter,
            bits: bits,
        }
    }
}

/// Read serial inputs from port and emulates key inputs
fn read_serial(port: &mut Box<dyn SerialPort>, bits_data: &Arc<AtomicU8>) {
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
