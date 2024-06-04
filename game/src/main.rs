mod constants;
mod controller_wrapper;
mod game;

use clap::Parser;
use controller_wrapper::ControllerWrapper;
use game::init::{init_game, InitGameOptions};

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
    /// Enables vsync or not? (Default: enabled in macos, disabled otherwise)
    #[arg(long)]
    vsync: Option<bool>,
    /// Price
    #[cfg(not(feature = "uncommercial"))]
    #[arg(long, default_value_t = 2)]
    price: u32,
}

#[cfg(feature = "uncommercial")]
macro_rules! price {
    ($args: expr) => {
        0
    };
}

#[cfg(not(feature = "uncommercial"))]
macro_rules! price {
    ($args: expr) => {
        $args.price
    };
}

fn main() {
    if cfg!(feature = "uncommercial") {
        println!("This is uncommercial version, only free play is available.");
    }

    let args = Args::parse();
    let options = InitGameOptions {
        fullscreen: !args.windowed,
        height: args.window_height,
        width: args.window_width,
        vsync: args.vsync.unwrap_or(if cfg!(target_os = "macos") {
            true
        } else {
            false
        }),
        price: price!(args),
    };

    let controller_wrapper = match args.controller_port {
        Some(controller_port) => ControllerWrapper::serial(controller_port),
        _ => ControllerWrapper::keyboard(),
    };
    init_game(controller_wrapper, options);
}
