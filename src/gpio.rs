use rpi_pal::gpio::{Gpio, OutputPin};
use std::thread;
use std::time::Duration;

pub struct GpioSetup {
    reset_pin: OutputPin,
    flash_pin: OutputPin,
}

impl GpioSetup {
    pub fn new(reset_pin_number: u8, flash_pin_number: u8) -> Self {
        let gpio = Gpio::new().unwrap();
        let reset_pin = gpio.get(reset_pin_number).unwrap().into_output();
        let flash_pin = gpio.get(flash_pin_number).unwrap().into_output();
        Self {
            reset_pin,
            flash_pin,
        }
    }

    pub fn enter_flashingmode(&mut self) {
        self.flash_pin.set_low();
        self.reset();
        thread::sleep(Duration::from_millis(100));
    }

    pub fn reset(&mut self) {
        self.reset_pin.set_low();
        thread::sleep(Duration::from_millis(100));
        self.reset_pin.set_high();
        thread::sleep(Duration::from_millis(100));
    }

    pub fn exit_flashingmode(&mut self) {
        self.flash_pin.set_high();
        self.reset();
    }

    
}
