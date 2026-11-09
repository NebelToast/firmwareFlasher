use espflash::{
    connection::{Connection, ResetAfterOperation, ResetBeforeOperation},
    flasher::Flasher,
    target::DefaultProgressCallback,
};
use serialport::UsbPortInfo;
use std::time::Duration;

pub struct FirmwareFlasher {
    flasher: Flasher,
}

impl FirmwareFlasher {
    pub fn new(connection: Connection) -> Self {
        let flasher = Flasher::connect(connection, true, true, false, None, Some(460_800))
            .expect("Connection struct was invalid");
        Self { flasher }
    }

    pub fn from_port(port_path: &str) -> Self {
        let serial = serialport::new(port_path, 115_200)
            .timeout(Duration::from_secs(3))
            .open_native()
            .expect("Port isn't valid");

        let dummy_port_info = UsbPortInfo {
            vid: 0,
            pid: 0,
            serial_number: None,
            manufacturer: None,
            product: None,
        };

        let connection = Connection::new(
            serial,
            dummy_port_info,
            ResetAfterOperation::NoReset,
            ResetBeforeOperation::NoReset,
            115_200,
        );

        Self::new(connection)
    }

    pub fn flash_firmware(
        mut self,
        addr: u32,
        data: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Writinh {} Bytes to adress {:#x}", data.len(), addr);
        self.flasher
            .write_bin_to_flash(addr, data, &mut DefaultProgressCallback)?;
        println!("Flash worked");
        Ok(())
    }
}
