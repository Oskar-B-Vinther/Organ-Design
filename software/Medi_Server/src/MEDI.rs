pub mod medi {
    use midly::Smf;
    use std::fs;
    use std::vec;

    pub struct song {
        bytes: Vec<u8>,
        readindex: u32,
    }

    impl song {
        pub fn new(file_name: String) -> Self {
            let bytes: Vec<u8> = fs::read("test-asset/Clementi.mid").unwrap();
            let readindex = 0;
            song { bytes, readindex }
        }

        pub fn send_next_event() {}
    }
}
