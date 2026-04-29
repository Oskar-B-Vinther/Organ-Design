
use std::io::{self, Read, Write};
use std::time::Duration;

fn main() {
    

    // 1. Open the port
    let mut port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(1000)) // Set a read timeout
        .open()
        .expect("Failed to open port");

    // 2. Send bytes
    let msg = b"";
    port.write_all(msg).expect("Write failed!");
    println!("Sent: {:?}", String::from_utf8_lossy(msg));

    // 3. Read bytes
    let mut buffer: [u8; 128] = [0; 128];
    match port.read(&mut buffer) {
        Ok(bytes_read) => {
            let data = &buffer[..bytes_read];
            println!("Received: {:?}", String::from_utf8_lossy(data));
        }
        Err(e) if e.kind() == io::ErrorKind::TimedOut => {
            eprintln!("Read timed out!");
        }
        Err(e) => eprintln!("Error reading: {:?}", e),
    }
}