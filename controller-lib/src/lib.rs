pub mod keyboard;
pub mod serial;

use bidrum_data_struct_lib::janggu::JangguInputState;

/// Bidrum Janggu Controller
pub trait JangguDevice {
    /// Reads janggu controller input state
    fn read_janggu_input_state(&self) -> JangguInputState;
}

/// Bidrum Coin/Bill Acceptor
pub trait CoinInputDevice {
    /// Reads unconsumed coin count
    fn get_unconsumed_coins(&self) -> u32;
    fn consume_coins(&mut self, coins: u32);
}
