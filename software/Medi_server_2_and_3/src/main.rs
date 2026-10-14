mod MEDI;

use serialport::SerialPort;
use std::time::Duration;
use std::{thread, time, vec};

fn main() {
    let path: &str = r"C:\Users\oskar\Oskars filer\Filer\andre projketer\Organ projket\Organ-Design\software\Medi_server_2_and_3\Medi.mid";
    let mut state_machine = App::new("com9", &path);

    //state_machine.print_config();

    let quit_program = false;
    while !quit_program {

        // state_machine.update();
    }
}

pub enum answer {
    Next,
    None,
    NoAnswer,
    Ping,
}

#[derive(Clone)]
pub enum message {
    Next_event(Vec<u8>),
    Start_Song(Vec<u8>),
    Ping,
    None,
}

pub struct App {
    port_name: String,
    baud_rate: u32,
    port: Option<Box<dyn SerialPort>>,

    isplaying: bool,
    pub song: MEDI::song,
    next_message: message,
}

impl App {
    pub fn new(port_name: &str, song_path: &str) -> Self {
        let mut song: MEDI::song = MEDI::song::new(song_path);

        let port_name = port_name.to_string();
        let baud_rate = 9600;
        let isplaying = false;

        /*
        let mut port: Box<dyn SerialPort> = serialport::new(&port_name, baud_rate)
            .timeout(Duration::from_millis(1000)) // Set a read timeout
            .open()
            .expect("Failed to open port");
        */

        let mut port = App::try_oppening_port(&port_name, baud_rate);

        let mut next_message = message::None;

        Self {
            isplaying,
            song,
            port,
            port_name,
            baud_rate,
            next_message,
        }
    }

    pub fn try_oppening_port(port_name: &String, baud_rate: u32) -> Option<Box<dyn SerialPort>> {
        let port_enum = match serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(1000)) // Set a read timeout
            .open()
        {
            Ok(port) => Option::Some(port),
            Err(e) => {
                println!("port could not open{}", e);
                Option::None
            }
        };

        return port_enum;
    }

    pub fn check_port(&mut self) {
        match &self.port {
            Option::Some(a) => {}
            Option::None => self.port = App::try_oppening_port(&self.port_name, self.baud_rate),
        }
    }

    pub fn send_message(&mut self, data: &[u8]) {
        match &mut self.port {
            Option::Some(port) => {
                println!("Send Data{:?}", data);
                if let Err(e) = port.write_all(data) {
                    //  println!("Failed to write to port: {}", e);
                }
            }
            Option::None => {
                // println!("tried Sending Data port {:?}", self.port_name);
            }
        }
    }

    pub fn receive_message(&mut self) -> Vec<u8> {
        match &mut self.port {
            Option::Some(port) => {
                let mut buffer: [u8; 128] = [0; 128];
                match port.read(&mut buffer) {
                    Ok(n) => buffer[..n].to_vec(),
                    Err(_) => Vec::new(),
                }
            }
            Option::None => {
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

    pub fn update_sate(&mut self, answer: Vec<u8>) -> answer {
        /*

        if answer.is_empty() {
            return answer::NoAnswer;
        }

        println!("revived: {}", answer[0]);

        match answer[0] {
            0x04 => {
                self.next_message = message::Next_event(self.song.next_config());
                match &self.port {
                    Option::Some(a) => self.isplaying = true,
                    _ => {}
                }
                answer::Next
            }

            0x86 => {
                println!("audiono aswered ping");
                answer::Ping
            }
            _ => answer::None,
        }
        */
        answer::None
    }

    pub fn do_because_of_state(&mut self) {

        /*
        let message_enum = self.next_message.clone();

        match message_enum {
            message::Next_event(message) => {
                self.print_config(&message);
                self.send_message(&message);
            }
            message::Ping => {
                self.send_message(&vec![0x00]);
            }
            message::Start_Song(start_song) => {
                println!("{}", start_song[0]);
                self.send_message(&start_song);
            }
            _ => {}
        }
        */
    }

    pub fn print_config(&mut self) {
        print!("config: ");

        let total_config: Vec<Vec<u8>> = self.song.config.clone();

        for j in 0..total_config.len() {
            let config = &total_config[j];
            for i in 0..4 {
                print! {" {:08b}",config[i]};
            }

            let time = [config[5], config[6], config[7], config[8]];
            let time_u32 = u32::from_be_bytes(time);
            print!("Delta time: {}", time_u32);
        }
    }

    pub fn update(&mut self) {
        // ------ // update state:
        self.check_port();
        let response = self.receive_message();
        self.update_sate(response);

        // ------ // Do because of that state
        self.do_because_of_state();
    }

    // debug functions
}
