use midly::MidiMessage;
use midly::Smf;
use midly::Timing;
use midly::num;
use midly::num::u15;
use std::error;
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
    pub isplaying: bool,
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

        let lower_bound = 40;
        let higher_bound = lower_bound + 31;
        let last_config = vec![0x00, 0x00, 0x00, 0x00];

        let organ_config: organ = organ {
            lower_bound,
            higher_bound,
            last_config,
        };
        let isplaying = false;

        song {
            isplaying,
            bytes,
            readindex,
            timeing,
            organ_config,
        }
    }

    pub fn next_event(&mut self) -> Vec<u8> {
        let next_event: Vec<u8> = Vec::new();

        let smf = Smf::parse(&self.bytes).unwrap();

        if self.readindex == smf.tracks[0].len() {
            self.isplaying = false;
            return vec![
                0x00 as u8, 0x00 as u8, 0x00 as u8, 0x00 as u8, 0x00 as u8, 0x00 as u8, 0x00 as u8,
            ];
        }

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
                    vec![0x00 as u8, 0x00 as u8]
                }
            },
            _ => {
                vec![0x00 as u8, 0x00 as u8]
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
        let mut first_play = true;

        'shouldterminate: loop {
            let mut next_event = self.next_event();
            //println!("Config: {:?}", next_event);

            let delta_time = vec![next_event[2], next_event[3], next_event[4], next_event[5]];

            if !self.isplaying {
                self.readindex = 0;
                panic!("done processing song");
                break 'shouldterminate;
            }
            if !first_play {
                for i in delta_time {
                    if i != 0x00 {
                        self.readindex = self.readindex - 1;
                        break 'shouldterminate;
                    }
                }
            }

            // let note_bytes = vec![0x00 as u8, 0x00 as u8, next_event[1], next_event[2]];
            // let medi_note: i32 = i32::from_be_bytes(note_bytes.try_into().unwrap());
            let medi_note: i32 = next_event[1] as i32;

            let state = match next_event[0] {
                0x90 => false,
                0x80 => true,
                0x00 => false,
                _ => panic!("wired note type: {:?}", next_event[0]),
            };

            next_config = self.set_medinote_in_config(medi_note, state, &mut next_config);
            first_play = false;
        }

        self.organ_config.last_config = next_config.clone();

        next_config
    }

    fn set_medinote_in_config(
        &mut self,
        medi_note: i32,
        on_off: bool,
        config: &mut Vec<u8>,
    ) -> Vec<u8> {
        if (medi_note < self.organ_config.lower_bound || medi_note > self.organ_config.higher_bound)
        {
            return config.clone();
        }

        let bourdy_fixed_value = medi_note - self.organ_config.lower_bound;

        let byte_number = (bourdy_fixed_value / 8 as i32) as usize;
        let byte_bit_number = (7 - (bourdy_fixed_value % 8)) as u8;

        config[byte_number] = Self::set_bit(config[byte_number], byte_bit_number, on_off);

        return config.to_vec();
    }

    fn set_bit(x: u8, idx: u8, b: bool) -> u8 {
        let mask = !(1 << idx);
        let flag = (b as u8) << idx;
        x & mask | flag
    }
}
