use std::collections::HashMap;
use std::fmt;

pub struct HexValue {
    pub bytes: Vec<u8>,
}

impl HexValue {
    pub fn from_bytes(bytes: &Vec<u8>) -> HexValue {
        HexValue {
            bytes: bytes.clone(),
        }
    }

    pub fn from_str(val: &str) -> Result<HexValue, &'static str> {
        // Verify that the input is hex.
        let val = val.to_lowercase();
        match val.chars().map(|c| HexValue::is_hex_char(c)).all(|b| b) {
            true => Ok(HexValue {
                bytes: Self::to_bytes(&val),
            }),
            false => Err("value contains illegal characters"),
        }
    }

    fn is_hex_char(c: char) -> bool {
        match c {
            'a'...'f' | '0'...'9' => true,
            _ => false,
        }
    }

    fn hex_to_byte(hex: char) -> u8 {
        match hex {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'a' => 10,
            'b' => 11,
            'c' => 12,
            'd' => 13,
            'e' => 14,
            'f' => 15,
            _ => panic!("{} is not a valid hex value", hex),
        }
    }

    fn to_bytes(val: &str) -> Vec<u8> {
        let mut bytes = vec![0; val.len() / 2];
        let hex = val.chars().enumerate();
        for (i, c) in hex {
            let byte_ind = i / 2;
            match i % 2 {
                0 => bytes[byte_ind] = Self::hex_to_byte(c),
                1 => bytes[byte_ind] = (bytes[byte_ind] << 4) | Self::hex_to_byte(c),
                _ => (),
            };
        }

        bytes
    }

    fn nibble_to_hex(nibble: u8) -> char {
        let alphabet = "0123456789abcdef";
        alphabet.chars().nth(nibble as usize).unwrap()
    }

    pub fn byte_to_hex(byte: u8) -> String {
        format!(
            "{}{}",
            Self::nibble_to_hex(byte >> 4),
            Self::nibble_to_hex(byte & 0x0f)
        )
    }

    fn to_string(&self) -> String {
        let mut out = String::with_capacity(self.bytes.len() * 2);
        for b in self.bytes.iter() {
            out.push_str(&format!(
                "{}{}",
                Self::nibble_to_hex((b & 0xf0) >> 4),
                Self::nibble_to_hex(b & 0x0f)
            ));
        }
        out
    }

    pub fn to_base64(&self) -> Base64Value {
        Base64Value::from_bytes(&self.bytes)
    }

    pub fn xor(&self, other: &HexValue) -> HexValue {
        assert!(self.bytes.len() == other.bytes.len());
        let mut new: Vec<u8> = Vec::with_capacity(self.bytes.len());
        for (b1, b2) in self.bytes.iter().zip(other.bytes.iter()) {
            new.push(*b1 ^ *b2);
        }

        HexValue::from_bytes(&new)
    }
}

impl PartialEq for HexValue {
    fn eq(&self, other: &HexValue) -> bool {
        self.bytes == other.bytes
    }
}

impl Eq for HexValue {}

impl fmt::Display for HexValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

const BASE64_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
lazy_static! {
    static ref BASE64_REVERSE_ALPHABET: HashMap<char, u8> = {
        let mut map = HashMap::new();
        for (i, c) in BASE64_ALPHABET.chars().enumerate() {
            map.insert(c, i as u8);
        }
        map
    };
}

#[derive(Debug)]
pub struct Base64Value {
    bytes: Vec<u8>,
}

impl Base64Value {
    pub fn from_bytes(bytes: &Vec<u8>) -> Base64Value {
        Base64Value {
            bytes: bytes.clone(),
        }
    }

    pub fn from_str(val: &str) -> Result<Base64Value, &'static str> {
        // Verify that the input is base 64.
        match val.chars().map(|c| Self::is_base64_char(c)).all(|b| b) {
            true => {
                let mut bytes: Vec<u8> = vec![0; ((val.len() as f32) * 6. / 8.).ceil() as usize];
                let mut curr_byte = bytes.len();
                for (i, c) in val.chars().rev().enumerate() {
                    // A base 64 character is six bits of information. In filling a byte vector,
                    // there's four cases:
                    //   - We are starting on a new byte, so we fill the lowest six bits.
                    //   - Six bits in the current byte are filled, so we fill two in it
                    //     and four in the next byte.
                    //   - Four bits in the current byte are filled, so we fill the remaining
                    //     four and then two in the next byte.
                    //   - Two bits in the current byte are filled, so we fill the remaining
                    //     six.
                    match i % 4 {
                        0 => {
                            curr_byte -= 1;
                            bytes[curr_byte] |= Self::char_to_byte(c)
                        }
                        1 => {
                            let byte = Self::char_to_byte(c);
                            bytes[curr_byte] |= (byte & 0x03) << 6;
                            curr_byte -= 1;
                            bytes[curr_byte] |= (byte & 0xfc) >> 2;
                        }
                        2 => {
                            let byte = Self::char_to_byte(c);
                            bytes[curr_byte] |= (byte & 0x0f) << 4;
                            curr_byte -= 1;
                            bytes[curr_byte] |= (byte & 0xf0) >> 4;
                        }
                        _ => {
                            // 3
                            bytes[curr_byte] |= Self::char_to_byte(c) << 2;
                        }
                    }
                }
                Ok(Base64Value { bytes: bytes })
            }
            false => Err("value contains illegal characters"),
        }
    }

    fn is_base64_char(c: char) -> bool {
        BASE64_REVERSE_ALPHABET.contains_key(&c)
    }

    fn char_to_byte(c: char) -> u8 {
        *BASE64_REVERSE_ALPHABET.get(&c).unwrap()
    }

    fn byte_to_char(byte: u8) -> char {
        return BASE64_ALPHABET.chars().nth(byte as usize).unwrap();
    }

    fn to_string(&self) -> String {
        let mut segments: Vec<u32> = vec![0; ((self.bytes.len() as f32) / 3.0).ceil() as usize];

        for (i, byte) in self.bytes.iter().rev().enumerate() {
            let byte: u32 = *byte as u32;
            segments[i / 3] |= byte << 8 * (i % 3);
        }

        let mut chars: Vec<char> = Vec::with_capacity(segments.len() * 4);
        for segment in segments {
            for shift in 0..4 {
                chars.push(Base64Value::byte_to_char(
                    ((segment >> 6 * shift) & 0x3f) as u8,
                ));
            }
        }

        chars.into_iter().rev().collect()
    }
}

impl PartialEq for Base64Value {
    fn eq(&self, other: &Base64Value) -> bool {
        self.bytes == other.bytes
    }
}

impl Eq for Base64Value {}

impl fmt::Display for Base64Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use conversions::*;

    #[test]
    fn test_hexvalue_from_bytes() {
        let hex = HexValue::from_bytes(&vec![171, 205]);
        assert_eq!(hex.bytes, vec![171, 205]);

        let hex = HexValue::from_bytes(&vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 240, 241, 242, 243, 244, 245,
            246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
        ]);
        assert_eq!(
            hex.bytes,
            vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 240, 241, 242, 243, 244, 245,
                246, 247, 248, 249, 250, 251, 252, 253, 254, 255
            ]
        );
    }

    #[test]
    fn test_hexvalue_from_str() {
        let hex = HexValue::from_str("abcd");
        assert!(hex.is_ok());
        assert_eq!(hex.unwrap().bytes, vec![171, 205]);

        let hex = HexValue::from_str("AB");
        assert!(hex.is_ok());
        assert_eq!(hex.unwrap().bytes, vec![171]);

        let hex = HexValue::from_str("abch");
        assert!(hex.is_err());

        let hex =
            HexValue::from_str("000102030405060708090a0b0c0d0e0ff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
        assert!(hex.is_ok());
        assert_eq!(
            hex.unwrap().bytes,
            vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 240, 241, 242, 243, 244, 245,
                246, 247, 248, 249, 250, 251, 252, 253, 254, 255
            ]
        );
    }

    #[test]
    fn test_hexvalue_is_hex_char() {
        for c in "0123456789abcdef".chars() {
            assert!(HexValue::is_hex_char(c));
        }
        for c in "ghijklmno".chars() {
            assert!(!HexValue::is_hex_char(c));
        }
    }

    #[test]
    fn test_hexvalue_to_byte() {
        let alphabet = "0123456789abcdef";
        for (i, c) in alphabet.chars().enumerate() {
            assert_eq!(HexValue::hex_to_byte(c), i as u8);
        }
    }

    #[test]
    fn test_hexvalue_nibble_to_hex() {
        let alphabet = "0123456789abcdef";
        for val in 0..16 {
            assert_eq!(
                HexValue::nibble_to_hex(val),
                alphabet.chars().nth(val as usize).unwrap()
            );
        }
    }

    #[test]
    fn test_byte_to_hex() {
        let alphabet = "0123456789abcdef";
        for val1 in 0..16 {
            for val2 in 0..16 {
                assert_eq!(
                    HexValue::byte_to_hex(val1 * 16 + val2),
                    format!(
                        "{}{}",
                        alphabet.chars().nth(val1 as usize).unwrap(),
                        alphabet.chars().nth(val2 as usize).unwrap()
                    )
                );
            }
        }
    }

    #[test]
    fn test_hexvalue_to_string() {
        assert_eq!(HexValue::from_bytes(&vec![171, 205]).to_string(), "abcd");

        let hex = HexValue::from_bytes(&vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 240, 241, 242, 243, 244, 245,
            246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
        ]);
        assert_eq!(
            hex.to_string(),
            "000102030405060708090a0b0c0d0e0ff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff"
        );
    }

    #[test]
    fn test_hexvalue_to_base64() {
        let hex = HexValue::from_str("0f").unwrap();
        assert_eq!(hex.to_base64(), Base64Value::from_str("P").unwrap());

        let hex = HexValue::from_str("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
        assert_eq!(
            hex.to_base64(),
            Base64Value::from_str(
                "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
            ).unwrap()
        );
    }

    #[test]
    fn test_hexvalue_xor() {
        let hex1 = HexValue::from_str("28ac").unwrap();
        let hex2 = HexValue::from_str("ccf8").unwrap();
        assert_eq!(hex1.xor(&hex2).bytes, vec![228, 84]);

        let hex1 = HexValue::from_str("1c0111001f010100061a024b53535009181c").unwrap();
        let hex2 = HexValue::from_str("686974207468652062756c6c277320657965").unwrap();
        let hex3 = HexValue::from_str("746865206b696420646f6e277420706c6179").unwrap();
        assert_eq!(hex1.xor(&hex2).bytes, hex3.bytes);
    }

    #[test]
    fn test_base64_from_bytes() {
        let b64 = Base64Value::from_bytes(&vec![171, 205]);
        assert_eq!(b64.bytes, vec![171, 205]);

        let b64 = Base64Value::from_bytes(&vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 240, 241, 242, 243, 244, 245,
            246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
        ]);
        assert_eq!(
            b64.bytes,
            vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 240, 241, 242, 243, 244, 245,
                246, 247, 248, 249, 250, 251, 252, 253, 254, 255
            ]
        );
    }

    #[test]
    fn test_base64_from_str() {
        let b64 = Base64Value::from_str("AgN4");
        assert!(b64.is_ok());
        assert_eq!(b64.unwrap().bytes, vec![2, 3, 120]);

        let b64 = Base64Value::from_str("AgN*");
        assert!(b64.is_err());
    }

    #[test]
    fn test_base64_is_base64_char() {
        for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".chars() {
            assert!(Base64Value::is_base64_char(c));
        }
        for c in "!@#$%^&*()".chars() {
            assert!(!Base64Value::is_base64_char(c));
        }
    }

    #[test]
    fn test_base64_char_to_byte() {
        for (i, c) in "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
            .chars()
            .enumerate()
        {
            assert_eq!(Base64Value::char_to_byte(c), i as u8);
        }
    }

    #[test]
    fn test_base64_byte_to_char() {
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        for val in 0..64 {
            assert_eq!(
                Base64Value::byte_to_char(val),
                alphabet.chars().nth(val as usize).unwrap()
            );
        }
    }

    #[test]
    fn test_base64_to_string() {
        let b64 = Base64Value::from_bytes(&vec![2, 3, 120]);
        assert_eq!(b64.to_string(), "AgN4");
    }
}
