use serialport::SerialPort;
use std::io;
use std::time::Duration;
use std::{thread, time};
//use Medi
mod MEDI;

pub enum answer {
    Next,
    None,
    Ping,
}

fn main() {
    let port_name = "COM9"; // Use "COM3" on Windows
    let baud_rate = 9600;

    let mut port: Box<dyn SerialPort> = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(1000)) // Set a read timeout
        .open()
        .expect("Failed to open port");

    let mut song = MEDI::song::new(
        "C:\\Users\\oskar\\Desktop\\Git\\Organ-Design\\software\\Medi_Server\\Medi.mid",
    );

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

            "ping" => {
                let msg = 0x01 as u8;
                port.write_all(&[msg]).expect("Write failed!");
                println!("Sent: {:?}", msg);

                thread::sleep(time::Duration::from_millis(1000)); // waits a little for answer 

                let answer = handle_answer(receive_message(&mut port));

                match answer {
                    answer::Ping => println!("answered"),
                    answer::None => println!("no answer"),
                    _ => panic!("# -conflicting messages"),
                }

                /*
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
                */
            }

            "play" => {
                let timebytes = song.timeing.to_be_bytes().to_vec(); // converts to bytes
                let timing = vec![0x51, timebytes[0], timebytes[1]];

                send_message(&mut port, &timing); // Starts the song

                for i in 0..100 {
                    let msg = song.next_event();
                    send_message(&mut port, &msg);
                    print!("Sent: {:?} \n ", msg);
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

pub fn send_message(port: &mut Box<dyn SerialPort>, data: &[u8]) {
    if let Err(e) = port.write_all(data) {
        eprintln!("Failed to write to port: {}", e);
    }
}

pub fn receive_message(port: &mut Box<dyn SerialPort>) -> Vec<u8> {
    let mut buffer: [u8; 128] = [0; 128];
    match port.read(&mut buffer) {
        Ok(n) => buffer[..n].to_vec(),
        Err(_) => Vec::new(),
    }
}

pub fn handle_answer(answer: Vec<u8>) -> answer {
    if answer.len() > 0 {
        match answer[0] {
            0x04 => answer::Next,
            0x86 => answer::Ping,
            _ => answer::None,
        };
    }
    answer::None // retuns none is no message is heard
}
