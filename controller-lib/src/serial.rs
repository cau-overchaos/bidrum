mod coin_device;
mod janggu_device;
mod serial_reader;

use coin_device::SerialCoinDevice;
use janggu_device::SerialJangguDevice;
use serial_reader::BidrumSerialReader;

pub fn new(serial_port: String) -> (SerialJangguDevice, SerialCoinDevice) {
    let serial_reader = BidrumSerialReader::new(serial_port);
    return (
        SerialJangguDevice::new(serial_reader.clone()),
        SerialCoinDevice::new(serial_reader.clone()),
    );
}
