use serialport::SerialPort;
use std::io;
use std::time::Duration;
use std::{thread, time};

use crate::MEDI::song;

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

struct App {
    //dynamic states
    isplaying: bool,
    should_send_next_event: bool,

    port_name: String,
    baud_rate: u32,
    port: Box<dyn SerialPort>,

    song: MEDI::song,
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

        Self {
            song,
            port,
            port_name,
            baud_rate,
            isplaying,
            should_send_next_event,
        }
    }

    pub fn send_message(&mut self, data: &[u8]) {
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

    pub fn handle_answer(&mut self, answer: Vec<u8>) -> answer {
        if answer.is_empty() {
            return answer::NoAnswer;
        }

        println!("revived: {}", answer[0]);

        match answer[0] {
            0x04 => {
                self.should_send_next_event = true;
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
        self.handle_answer(response);
    }
}
