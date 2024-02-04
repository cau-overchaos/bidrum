mod game;
mod janggu_keyboard;
mod serial;

use std::{
    sync::{atomic::AtomicU8, Arc},
    thread::{self, sleep},
    time::Duration,
};

use clap::Parser;
use game::init::{init_game, InitGameOptions};
use janggu_keyboard::read_janggu_key_loop;

use crate::serial::read_serial_loop;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port of janggu controller
    #[arg(short, long)]
    controller_port: Option<String>,
    /// Window width (default value: width of current display mode)
    #[arg(long)]
    window_width: Option<u32>,
    /// Window height (default value: width of current display mode)
    #[arg(long)]
    window_height: Option<u32>,
    /// Runs game in non-ullscreen
    #[arg(short, long)]
    windowed: bool,
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
        _ => {
            println!("Controller port not provided! Reading keyboard....");

            let ptr = bits_arc.clone();
            thread::spawn(move || {
                read_janggu_key_loop(ptr);
            });
        }
    }

    let ptr = bits_arc.clone();
    init_game(
        ptr,
        InitGameOptions {
            fullscreen: !args.windowed,
            height: args.window_height,
            width: args.window_width,
        },
    );
}
