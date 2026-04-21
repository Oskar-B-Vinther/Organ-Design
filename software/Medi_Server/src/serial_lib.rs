


use std::io::{Read, Write};
use std::time::Duration;

struct SerialConnection {
    port: Box<dyn serialport::SerialPort>,
}

impl SerialConnection {
    fn new(port_name: &str, baud_rate: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(1000))
            .open()?;

        Ok(SerialConnection { port })
    }

    fn write_bytes(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.port.write_all(data)?;
        Ok(())
    }

    fn read_bytes(&mut self, buffer: &mut [u8]) -> Result<usize, Box<dyn std::error::Error>> {
        let bytes_read = self.port.read(buffer)?;
        Ok(bytes_read)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Change this to your Arduino port:
    // Linux: "/dev/ttyUSB0" or "/dev/ttyACM0"
    // Windows: "COM3"
    let port_name = "/dev/ttyUSB0";
    let baud_rate = 9600;

    let mut connection = SerialConnection::new(port_name, baud_rate)?;

    // Write example
    let data_to_send = b"Hello Arduino\n";
    connection.write_bytes(data_to_send)?;

    // Read example
    let mut buffer = [0u8; 64];
    let bytes_read = connection.read_bytes(&mut buffer)?;

    println!("Read {} bytes: {:?}", bytes_read, &buffer[..bytes_read]);

    Ok(())
}