mod game;
mod serial;

use std::{
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

            thread::spawn(move || {
                read_serial_loop(port);
            });
        }
        _ => {}
    }

    init_game();
}
