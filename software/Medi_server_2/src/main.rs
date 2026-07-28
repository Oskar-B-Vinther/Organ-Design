use serialport::SerialPort;
use std::time::Duration;
use std::{io, io::BufRead, string};
use std::{thread, time};

use crate::mpsc::Receiver;
use std::sync::mpsc;

mod MEDI;
mod test;
//use Medi

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    std::thread::spawn(move || {
        let stdin = std::io::stdin();

        for line in stdin.lock().lines() {
            tx.send(line.unwrap()).unwrap();
        }
    });

    let mut state_machine = App::new(
        "com9",
        "C:\\Users\\oskar\\Desktop\\Organ projket\\Organ-Design\\software\\Medi_server_2\\Medi.mid",
        rx,
    );
    let quit_program = false;

    state_machine.song.isplaying = true;
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

    rx: Receiver<String>,
    port: Port,

    song: MEDI::song,

    next_message: message,
}

impl App {
    pub fn new(port_name: &str, song_path: &str, rx: Receiver<String>) -> Self {
        let mut song: MEDI::song = MEDI::song::new(song_path);

        let port_name = port_name.to_string();
        let baud_rate = 9600;
        let isplaying = false;
        let should_send_next_event = false;

        /*
        let mut port: Box<dyn SerialPort> = serialport::new(&port_name, baud_rate)
            .timeout(Duration::from_millis(1000)) // Set a read timeout
            .open()
            .expect("Failed to open port");
        */

        let mut port = App::try_oppening_port(&port_name, baud_rate);

        let mut next_message = message::None;

        Self {
            rx,
            song,
            port,
            port_name,
            baud_rate,
            next_message,
        }
    }

    pub fn try_oppening_port(port_name: &String, baud_rate: u32) -> Port {
        let port_enum = match serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(1000)) // Set a read timeout
            .open()
        {
            Ok(port) => Port::some(port),
            Err(e) => {
                println!("port could not open{}", e);
                Port::none
            }
        };

        return port_enum;
    }

    pub fn check_port(&mut self) {
        match &self.port {
            Port::some(a) => {}
            Port::none => self.port = App::try_oppening_port(&self.port_name, self.baud_rate),
        }
    }

    pub fn send_message(&mut self, data: &[u8]) {
        match &mut self.port {
            Port::some(port) => {
                println!("Send Data{:?}", data);
                if let Err(e) = port.write_all(data) {
                    //  println!("Failed to write to port: {}", e);
                }
            }
            Port::none => {
                // println!("tried Sending Data port {:?}", self.port_name);
            }
        }
    }

    pub fn receive_message(&mut self) -> Vec<u8> {
        match &mut self.port {
            Port::some(port) => {
                let mut buffer: [u8; 128] = [0; 128];
                match port.read(&mut buffer) {
                    Ok(n) => buffer[..n].to_vec(),
                    Err(_) => Vec::new(),
                }
            }
            Port::none => {
                /*
                println!(
                    "tried reciving Data port, could not acsess port: {:?}",
                    self.port_name
                );
                */
                Vec::new()
            }
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

    pub fn read_cmd_input() {}

    pub fn update(&mut self) {
        let response = self.receive_message();
        self.revice_message_and_update_sate(response);
        self.check_port();

        if self.song.isplaying {
            let bytes = self.song.next_config();
            //println!("config{:?}", self.song.next_config());

            for i in bytes {
                print!("config: {:08b}", i);
            }
        }
        



    }
}
