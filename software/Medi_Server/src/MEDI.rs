use midly::MidiMessage;
use midly::Smf;
use midly::Timing;
use midly::num;
use midly::num::u15;
use std::fs;
use std::io;
use std::vec;

pub struct song {
    bytes: Vec<u8>,
    readindex: usize,
    timeing: u15,
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

        let readindex = 0;
        song {
            bytes,
            readindex,
            timeing,
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
                    vec![0x90 as u8, key.as_int() as u8]
                }
                MidiMessage::NoteOn { key, vel } => {
                    vec![0x80 as u8, key.as_int() as u8]
                }
                _ => {
                    vec![0x80 as u8]
                }
            },
            _ => {
                vec![0x80 as u8]
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
}
