mod serial;

use std::{
    sync::{mpsc, Arc, Mutex, RwLock},
    thread::{self, sleep},
    time::Duration,
};

use clap::Parser;

use crate::{
    game::init_game,
    serial::{empty_controller_state, read_serial_loop, ControllerState},
};

mod game;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port of janggu controller
    #[arg(short, long)]
    controller_port: String,
}

fn main() {
    let args = Args::parse();
    println!("Openning serial port {}", args.controller_port);

    let port = serialport::new(args.controller_port, 9600)
        .timeout(Duration::from_millis(20))
        .open()
        .expect("Failed to open port");

    println!("Waiting 3 seconds (Arduino needs time for serial initialization)");
    sleep(Duration::from_millis(3000));
    println!("Waited 3 seconds!");

    let controller_state = Arc::new(RwLock::new(empty_controller_state()));
    let controller_state_lock_for_read_loop = Arc::clone(&controller_state);
    thread::spawn(move || {
        read_serial_loop(port, controller_state_lock_for_read_loop);
    });

    let controller_state_lock_for_game_loop = Arc::clone(&controller_state);

    init_game(controller_state_lock_for_game_loop);
}
