fn set_bit(x: u8, idx: u8, b: bool) -> u8 {
    let mask = !(1 << idx);
    let flag = (b as u8) << idx;
    x & mask | flag
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_set_bit() {
        assert_eq!(set_bit(0x00 as u8, 0x00 as u8, true as bool), 0x01 as u8);
    }
}
