use midly::MidiMessage;
use midly::Smf;
use midly::Timing;
use midly::num;
use midly::num::u15;
use std::fs;
use std::io;
//use std::ops::Complete;
use std::ops::ControlFlow::Break;
use std::vec;

use crate::message::Next_event;

#[derive(Clone)]
pub struct organ {
    lower_bound: i32,
    higher_bound: i32,
    last_config: Vec<u8>,
}

pub struct song {
    bytes: Vec<u8>,
    readindex: usize,
    pub timeing: u16,
    organ_config: organ,
}

impl song {
    pub fn new(file_name: &str) -> Self {
        let bytes: Vec<u8> = fs::read(file_name).unwrap();
        let smf = Smf::parse(&bytes).unwrap();
        let timeing_e: midly::Timing = smf.header.timing;
        let timeing: u15 = match timeing_e {
            Timing::Metrical(timeing) => timeing,

            _ => {
                print!("Wrong timing format");
                panic!()
            }
        };

        let timeing: u16 = timeing.into(); // convert u15 to u16

        let readindex = 0;

        let lower_bound = 32;
        let higher_bound = 64;
        let last_config = vec![0x00, 0x00, 0x00, 0x00];

        let organ_config: organ = organ {
            lower_bound,
            higher_bound,
            last_config,
        };

        song {
            bytes,
            readindex,
            timeing,
            organ_config,
        }
    }

    pub fn next_event(&mut self) -> Vec<u8> {
        let next_event: Vec<u8> = Vec::new();

        let smf = Smf::parse(&self.bytes).unwrap();

        let next_event = smf.tracks[0][self.readindex];

        let event = next_event.kind;

        let mut note_event = match event {
            midly::TrackEventKind::Midi { channel, message } => match message {
                MidiMessage::NoteOff { key, vel } => {
                    println!("test");
                    vec![0x90 as u8, key.as_int() as u8]
                }
                MidiMessage::NoteOn { key, vel } => {
                    if vel == 0 {
                        vec![0x90 as u8, key.as_int() as u8]
                    } else {
                        vec![0x80 as u8, key.as_int() as u8]
                    }
                }
                _ => {
                    vec![0x00 as u8]
                }
            },
            _ => {
                vec![0x00 as u8]
            }
        };
        let time = next_event.delta.as_int();
        let time_bytyfied = time.to_be_bytes().to_vec();
        note_event.push(time_bytyfied[0]);
        note_event.push(time_bytyfied[1]);
        note_event.push(time_bytyfied[2]);
        note_event.push(time_bytyfied[3]);

        self.readindex += 1;
        note_event
    }

    pub fn next_config(&mut self) -> Vec<u8> {
        let mut next_config: Vec<u8> = self.organ_config.last_config.clone();

        let mut should_terminate = false;
        while !should_terminate {
            let mut next_event = self.next_event();

            let note_bytes = vec![next_event[1], next_event[2]];
            let value = i32::from_be_bytes(note_bytes.try_into().unwrap());

            next_config = set_medinote_in_config();

            let delta_time = vec![
                next_config[2],
                next_config[3],
                next_config[4],
                next_config[5],
            ];
            should_terminate = true;
            for i in delta_time {
                if i == !0x00 {
                    should_terminate = false;
                }
            }
        }

        self.organ_config.last_config = next_config.clone();

        next_config
    }

    fn set_medinote_in_config(self, medi_note: i32, on_off: bool, config: &mut Vec<u8>) -> Vec<u8> {
        let bourdy_fixed_value =
            medi_note - (self.organ_config.higher_bound - self.organ_config.lower_bound);
        let byte_number = (bourdy_fixed_value / 4 as i32) as usize;
        let byte_bit_number = (bourdy_fixed_value % 8) as u8;

        config[byte_number] = Self::set_bit(config[byte_number], byte_bit_number, on_off);

        return config.to_vec();
    }

    fn set_bit(x: u8, idx: u8, b: bool) -> u8 {
        let mask = !(1 << idx);
        let flag = (b as u8) << idx;
        x & mask | flag
    }
}
