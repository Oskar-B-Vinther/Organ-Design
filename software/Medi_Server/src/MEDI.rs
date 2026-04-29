pub mod medi {
    use midly::Smf;
    use std::fs;
    use std::vec;

    pub struct song {
        events: Vec<Event>,
        temp: u8,
    }

    impl song {
        pub fn new(file_name: String) -> Self {
            let events = loadMediFile(file_name);
            let temp = 8 as u8;

            Self { events, temp }
        }
    }

    pub struct Event {
        note: u8,
        onOff: bool,
    }

    impl Event {
        pub fn new(note: u8, onOff: bool) -> Self {
            Self { note, onOff }
        }
    }

    pub fn loadMediFile(file_name: String) -> Vec<Event> {
        let events: Vec<Event> = Vec::new();

        let bytes = fs::read("test-asset/Clementi.mid").unwrap();
        let smf = Smf::parse(&bytes).unwrap();
        let track = &smf.tracks[0];
        let L = track.len();

        for i in 0..L {
            //events.push(Event::new());
        }
        vec![Event::new(20, false)] // place holder
    }
}
