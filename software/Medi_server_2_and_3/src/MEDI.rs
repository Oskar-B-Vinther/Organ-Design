use midly::{MidiMessage, Smf, Timing, num::u15};
use std::ops::ControlFlow::Break;
use std::{error, fs, io, ops::Index, result, usize, vec};

#[derive(Clone)]
pub struct organ {
    lower_bound: u8,
    higher_bound: u8,
}

pub struct song {
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
        let config = song::convert_format_to_config(&bytes, &organ_config);

        song {
            isplaying,
            readindex,
            timeing,
            organ_config,
            config,
        }
    }

    pub fn convert_format_to_config(bytes: &Vec<u8>, organ_config: &organ) -> Vec<Vec<u8>> {
        let mut config: Vec<Vec<u8>> = vec![vec![0x00, 0x00, 0x00, 0x00]];
        let mut smf: Smf<'_> = Smf::parse(bytes).unwrap();

        let mut run = true;
        let mut index = 0;

        while run {
            match song::next_config(&smf, &config[config.len() - 1], &index, organ_config) {
                Ok(next) => {
                    let (next_event, new_index) = next;
                    index = new_index;

                    config.push(next_event)
                }

                Err(a) => {
                    print!("Stoped phrasing");
                    run = false;
                }
            }
        }

        config
    }

    pub fn next_config(
        smf: &Smf<'_>,
        last_config: &Vec<u8>,
        last_index: &usize,
        organ_config: &organ,
    ) -> Result<(Vec<u8>, usize), String> {
        let mut config: Vec<u8> = last_config.clone();

        let mut first_play = true;

        // computre based on the last config:
        let mut index = last_index.clone();

        'config_builder: loop {
            let next_event = match song::get_event(smf, index) {
                Some(a) => a,
                None => return Result::Err(("vvvv").to_string()),
            };

            if song::is_zerro(&next_event[2..5].to_vec()) == true {
                // if the time chnage is zerro it is the same event and therefore multple event can happen. When the delta time is not 0 it must be a new event.
                break 'config_builder;
            } else {
                index = index + 1;
            }

            // adds the change to the previus config
            config = song::add_event(&next_event, &config, organ_config)
        }
        //----------------------------------//

        Result::Ok((config, index))
    }

    pub fn is_zerro(list: &Vec<u8>) -> bool {
        let mut is_true = true;
        for i in 0..list.len() {
            if !(list[i] == 0x00) {
                is_true = false;
            }
        }
        is_true
    }

    pub fn add_event(event: &Vec<u8>, config: &Vec<u8>, organ_config: &organ) -> Vec<u8> {
        let mut config_evnt: Vec<u8> = config[0..3].to_vec();

        let mut config_time: Vec<u8> = config[3..7].to_vec();

        let onOrOff = match event[0] {
            0x90 => true,
            0x80 => false,
            _ => panic!("wrong events in medi phrasing!!!"),
        };

        let note = event[1];

        config_evnt = song::set_medinote_in_config(note, onOrOff, &mut config_evnt, organ_config);

        config_evnt.append(&mut config_time); // add the time again
        config_evnt
    }

    pub fn get_event(smf: &Smf<'_>, index: &usize) -> Option<Vec<u8>> {
        // Gives the next medi event at "index" in SMF's
        // let next_event: Vec<u8> = Vec::new();

        let next_event = match smf.tracks[0].get(index.clone()) {
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

        // Returns in format with the follong bytes: IDByte_0, Key_byte_0, time_0, time_1. time_2, time_3
        // Id byte will be 0x80 for on and 0x90 for off

        Option::Some(note_event)
    }

    fn set_medinote_in_config(
        medi_note: u8,
        on_off: bool,
        config: &mut Vec<u8>,
        organ_config: &organ,
    ) -> Vec<u8> {
        if (medi_note < organ_config.lower_bound || medi_note > organ_config.higher_bound) {
            return config.clone();
        }

        let bourdy_fixed_value: u8 = medi_note - organ_config.lower_bound;

        let byte_number = ((bourdy_fixed_value / 8) as i32) as usize;
        let byte_bit_number = (7 - (bourdy_fixed_value % 8)) as u8;

        config[byte_number] = song::set_bit(config[byte_number], byte_bit_number, on_off);

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
