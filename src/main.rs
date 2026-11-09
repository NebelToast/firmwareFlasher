mod flasher;
mod gpio;
mod monitor;

use flasher::FirmwareFlasher;
use gpio::GpioSetup;
use monitor::SerialMonitor;
use std::fs;

fn main() {
    let mut gpio = GpioSetup::new(18, 17);
    gpio.enter_flashingmode();
    let flasher = FirmwareFlasher::from_port("/dev/serial0");
    let bin_data = fs::read("firmware.bin").expect("Couldnt read firmware");
    let flash_offset: u32 = 0x10000;
    flasher
        .flash_firmware(flash_offset, &bin_data)
        .expect("Flashing didnt work");
    gpio.exit_flashingmode();
    let monitor = SerialMonitor::new();
    monitor.serial_loop().expect("Serial loop failed");
}
