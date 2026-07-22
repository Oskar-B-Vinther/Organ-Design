use serialport::SerialPort;
use std::io;
use std::time::Duration;
use std::{thread, time};

use crate::MEDI::song;
use crate::answer::Next;

//use Medi
mod MEDI;

fn main() {
    let mut state_machine = App::new(
        "com9",
        "C:\\Users\\oskar\\Desktop\\Git\\Organ-Design\\software\\Medi_Server\\Medi.mid",
    );
    let quit_program = false;
    while !quit_program {
        state_machine.update();
    }

    //let port_name = "COM9"; // Use "COM3" on Windows
}

pub enum answer {
    Next,
    None,
    NoAnswer,
    Ping,
}

pub enum message {
    Next_event(Vec<u8>),
    Ping,
    None,
}

pub enum Port {
    some(Box<dyn SerialPort>),
    none,
}

struct App {
    port_name: String,
    baud_rate: u32,

    port: Box<dyn SerialPort>,

    song: MEDI::song,
    isplaying: bool,

    next_message: message,
}

impl App {
    pub fn new(port_name: &str, song_path: &str) -> Self {
        let mut song: MEDI::song = MEDI::song::new(song_path);

        let port_name = port_name.to_string();
        let baud_rate = 9600;
        let isplaying = false;
        let should_send_next_event = false;

        let mut port: Box<dyn SerialPort> = serialport::new(&port_name, baud_rate)
            .timeout(Duration::from_millis(1000)) // Set a read timeout
            .open()
            .expect("Failed to open port");

        
        };

        let next_message = message::None;

        Self {
            song,
            port,
            port_name,
            baud_rate,
            isplaying,
            next_message,
        }

    }


    fn try_oppening_port(&self) -> Port {
        let mut port_enum = match serialport::new(&self.port_name, self.baud_rate)
            .timeout(Duration::from_millis(1000)) // Set a read timeout
            .open()
        {
            Ok(port) => Port::some(port),
            Err(e) => {
                println!("port could not open{}", e);
                Port::none
            }



    }

    pub fn send_message(&mut self, data: &[u8]) {
        println!("Send Data{:?}", data);

        if let Err(e) = self.port.write_all(data) {
            eprintln!("Failed to write to port: {}", e);
        }
    }

    pub fn receive_message(&mut self) -> Vec<u8> {
        let mut buffer: [u8; 128] = [0; 128];
        match self.port.read(&mut buffer) {
            Ok(n) => buffer[..n].to_vec(),
            Err(_) => Vec::new(),
        }
    }

    pub fn revice_message_and_update_sate(&mut self, answer: Vec<u8>) -> answer {
        if answer.is_empty() {
            return answer::NoAnswer;
        }

        println!("revived: {}", answer[0]);

        match answer[0] {
            0x04 => {
                self.next_message = message::Next_event(vec![1, 2, 3]);
                answer::Next
            }

            0x86 => {
                println!("audiono aswered ping");
                answer::Ping
            }
            _ => answer::None,
        }
    }

    pub fn update(&mut self) {
        let response = self.receive_message();
        self.revice_message_and_update_sate(response);
    }
}
