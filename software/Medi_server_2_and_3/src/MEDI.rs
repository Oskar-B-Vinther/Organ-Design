use midly::{MidiMessage, Smf, Timing, num::u15};
use std::{error, fs, io, ops::Index, result, vec};

use crate::{Port::some, message::Next_event};
use std::ops::ControlFlow::Break;

#[derive(Clone)]
pub struct organ {
    lower_bound: i32,
    higher_bound: i32,
}

pub struct song {
    bytes: Vec<u8>,
    readindex: usize,
    pub timeing: u16,
    organ_config: organ,
    pub isplaying: bool,

    config: Vec<Vec<u8>>,
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

        let organ_config: organ = organ {
            lower_bound,
            higher_bound,
        };

        let isplaying = false;
        let config = song::convert_format_to_config(&bytes);

        song {
            isplaying,
            bytes,
            readindex,
            timeing,
            organ_config,
            config,
        }
    }

    pub fn convert_format_to_config(bytes: &Vec<u8>) -> Vec<Vec<u8>> {
        let mut config: Vec<Vec<u8>> = vec![vec![0x00, 0x00, 0x00, 0x00]];
        let mut smf: Smf<'_> = Smf::parse(bytes).unwrap();

        let mut run = true;

        while run {
            let next_config: Result<Vec<u8>, String> =
                Self::next_config(&smf, &config[config.len() - 1]);
            match next_config {
                Ok(next) => config.push(next),
                Err(a) => {
                    run = false;
                }
            }
        }

        config
    }

    pub fn next_config(smf: &Smf<'_>, last_config: &Vec<u8>, last_index: usize) -> Result<Vec<u8>, String> {

        let mut next_config: Vec<u8> = last_config.clone();

        let mut first_play = true;

        let mut next_cofig_time: Vec<u8> = vec![];

         let index = 0;  
        'shouldterminate: loop {
            let mut next_event: Vec<u8> = match song::next_event(smf, Index) {
                Some(a) => a
                None => return Result::ok( next_cofig_time)
            };

            //println!("Config: {:?}", next_event);

            let delta_time = vec![next_event[2], next_event[3], next_event[4], next_event[5]];

            if !first_play {
                for i in &delta_time {
                    if *i != 0x00 {
                        index = self.readindex - 1;
                        next_cofig_time = delta_time;
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

        next_config.extend(next_cofig_time);

        next_config
    }

    pub fn next_event(smf: Smf<'_>, index: usize) -> Option<Vec<u8>> {
        // Gives the next medi event at "index" in SMF's
        let next_event: Vec<u8> = Vec::new();

        let next_event = match smf.tracks[0].get(index) {
            Some(a) => a,
            None => return Option::None,
        };

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

        Option::Some(note_event)
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

    pub fn start_song(&mut self) {
        self.isplaying = true;
    }
}
