use serialport::TTYPort;
use std::{
    io::{self, Read, Write},
    time::Duration,
};

pub struct SerialMonitor {
    port: TTYPort,
    stdout: io::Stdout,
    buf: [u8; 1024],
}

impl SerialMonitor {
    pub fn new() -> Self {
        Self::from_port("/dev/serial0", 115_200)
    }

    pub fn from_port(port_name: &str, baud_rate: u32) -> Self {
        let port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(200))
            .open_native()
            .unwrap();

        let stdout = io::stdout();
        let buf = [0u8; 1024];
        Self { port, stdout, buf }
    }

    pub fn serial_loop(mut self) {
        loop {
            match self.port.read(&mut self.buf) {
                Ok(n) if n > 0 => {
                    self.stdout.write_all(&self.buf[..n]).unwrap();
                    self.stdout.flush().unwrap();
                }
                Ok(_) => {}
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {}
                Err(e) => {
                    eprintln!("Error: {e}");
                    break;
                }
            }
        }
    }
}

impl Default for SerialMonitor {
    fn default() -> Self {
        Self::new()
    }
}
