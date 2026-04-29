use std::io;
use std::time::Duration;

fn main() {
    let port_name = "COM9"; // Use "COM3" on Windows
    let baud_rate = 9600;

    let mut port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(1000)) // Set a read timeout
        .open()
        .expect("Failed to open port");

    // able to turn off the program.
    let mut keep_running = true;

    while keep_running {
        println!("# - Input Comand:");
        let mut comand = String::new();
        let input_result = io::stdin().read_line(&mut comand);

        match input_result {
            Ok(_t) => println!("# - loaded input: Sucsess"),
            Err(_error) => panic!("# - failed loading input"),
        }

        let comand = comand.trim();
        match comand {
            "Test" => println!("# - sucsess"),

            "pingPort" => {
                let msg = 0x01 as u8;
                port.write_all(&[msg]).expect("Write failed!");
                println!("Sent: {:?}", msg);

                let mut buffer: [u8; 128] = [0; 128];
                match port.read(&mut buffer) {
                    Ok(bytes_read) => {
                        let data = &buffer[..bytes_read];
                        println!("Received: {:?}", data);
                    }
                    Err(e) if e.kind() == io::ErrorKind::TimedOut => {
                        eprintln!("Read timed out!");
                    }
                    Err(e) => eprintln!("Error reading: {:?}", e),
                }
            }

            "exit" => {
                println!("# - Qutting Prograimg");
                keep_running = false
            }
            _ => println!("# - Invalid comand: {comand}\n\n"),
        }
    }
}
