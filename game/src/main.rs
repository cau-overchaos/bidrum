mod game;
mod serial;

use std::{
    sync::{atomic::AtomicU8, Arc},
    thread::{self, sleep},
    time::Duration,
};

use clap::Parser;
use game::init::init_game;

use crate::serial::read_serial_loop;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port of janggu controller
    #[arg(short, long)]
    controller_port: Option<String>,
}

fn main() {
    let args = Args::parse();
    let bits = AtomicU8::new(0);
    let bits_arc = Arc::new(bits);
    match args.controller_port {
        Some(controller_port) => {
            println!("Openning serial port {}", controller_port);

            let port = serialport::new(controller_port, 9600)
                .timeout(Duration::from_millis(20))
                .open()
                .expect("Failed to open port");

            println!("Waiting 3 seconds (Arduino needs time for serial initialization)");
            sleep(Duration::from_millis(3000));
            println!("Waited 3 seconds!");

            let ptr = bits_arc.clone();
            thread::spawn(move || {
                read_serial_loop(port, ptr);
            });
        }
        _ => {}
    }

    let ptr = bits_arc.clone();
    init_game(ptr);
}
