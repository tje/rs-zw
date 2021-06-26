use std::iter::FromIterator;

pub const ZW_0: char = '\u{200c}';
pub const ZW_1: char = '\u{200b}';

fn str_to_bin(s: &str) -> String {
    s.as_bytes()
        .iter()
        .map(|b| format!("{:08b}", b))
        .collect::<Vec<_>>()
        .concat()
}

fn bin_to_zw(s: &str) -> String {
    let chars = s.chars()
        .filter_map(|c| match c {
            '0' => Some(ZW_0),
            '1' => Some(ZW_1),
            _ => None,
        })
        .collect::<Vec<_>>();

    String::from_iter(chars)
}

pub fn encode(s: &str) -> String {
    let bin = str_to_bin(s);
    bin_to_zw(&bin)
}

fn bin_to_str(s: &str) -> String {
    let out: Vec<u8> = s.chars()
        .collect::<Vec<char>>()
        .chunks(8)
        .filter_map(|b| {
            let s = String::from_iter(b);
            u8::from_str_radix(&s, 2).ok()
        })
        .collect();

    std::str::from_utf8(&out)
        .unwrap()
        .to_owned()
}

fn zw_to_bin(s: &str) -> String {
    let chars = s.chars()
        .filter_map(|c| match c {
            ZW_0 => Some('0'),
            ZW_1 => Some('1'),
            _ => None,
        })
        .collect::<Vec<_>>();
    String::from_iter(chars)
}

pub fn decode(s: &str) -> String {
    let bin = zw_to_bin(s);
    bin_to_str(&bin)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_encode() {
        assert_eq!(encode("One"), "‌​‌‌​​​​‌​​‌​​​‌‌​​‌‌​‌​");
    }

    #[test]
    fn bin_test() {
        assert_eq!(str_to_bin("Hello"), "0100100001100101011011000110110001101111");
    }

    #[test]
    fn full_circle() {
        let s = "Test! 🍔";
        let enc = encode(s);
        let dec = decode(&enc);
        assert_eq!(dec, s);
    }

    #[test]
    fn encode_empty() {
        assert_eq!(encode(""), "");
    }

    #[test]
    fn decode_empty() {
        assert_eq!(decode(""), "");
    }

    #[test]
    fn decode_invalid() {
        assert_eq!(decode("asdf"), "");
    }

    #[test]
    fn double_cycle() {
        let enc = encode("Test");
        let enc2 = encode(&enc);
        let dec2 = decode(&enc2);
        let dec = decode(&dec2);

        assert_eq!(dec, "Test");
        assert_ne!(enc, enc2);
        assert_eq!(enc, dec2);
    }
}
