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

fn nibble_to_hex(nibble: u8) -> char {
    let alphabet = "0123456789abcdef";
    alphabet.chars().nth(nibble as usize).unwrap()
}

pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let mut bytes = vec![0; hex.len() / 2];
    let hex = hex.chars().enumerate();
    for (i, c) in hex {
        let byte_ind = i / 2;
        match i % 2 {
            0 => bytes[byte_ind] = hex_to_byte(c),
            1 => bytes[byte_ind] = (bytes[byte_ind] << 4) | hex_to_byte(c),
            _ => (),
        };
    }

    bytes
}

fn int_to_base64(val: u32) -> char {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    return alphabet.chars().nth(val as usize).unwrap();
}

pub fn bytes_to_hex(bytes: &Vec<u8>) -> String {
    let mut hex: Vec<char> = Vec::with_capacity(2 * bytes.len());
    for byte in bytes.iter() {
        hex.push(nibble_to_hex(byte >> 4));
        hex.push(nibble_to_hex(byte & 15));
    }

    hex.iter().collect()
}

fn bytes_to_base64(bytes: &Vec<u8>) -> String {
    let mut segments: Vec<u32> = vec![0; ((bytes.len() as f32) / 3.0).ceil() as usize];

    for (i, byte) in bytes.iter().rev().enumerate() {
        let byte: u32 = *byte as u32;
        segments[i / 3] |= byte << 8 * (i % 3);
    }

    let mut base64: Vec<char> = Vec::with_capacity(segments.len() * 4);
    for segment in segments {
        for shift in 0..4 {
            base64.push(int_to_base64((segment >> 6 * shift) & 63));
        }
    }

    base64.into_iter().rev().collect()
}

pub fn hex_to_base64(hex: &str) -> String {
    bytes_to_base64(&hex_to_bytes(hex))
}

#[cfg(test)]
mod tests {
    use conversions::*;

    #[test]
    fn test_hex_to_byte() {
        let alphabet = "0123456789abcdef";
        for (i, c) in alphabet.chars().enumerate() {
            assert_eq!(hex_to_byte(c), i as u8);
        }
    }

    #[test]
    fn test_nibble_to_hex() {
        let alphabet = "0123456789abcdef";
        for val in 0..16 {
            assert_eq!(
                nibble_to_hex(val),
                alphabet.chars().nth(val as usize).unwrap()
            );
        }
    }

    #[test]
    fn test_hex_to_bytes() {
        assert_eq!(
            hex_to_bytes("000102030405060708090a0b0c0d0e0ff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff"),
            vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 240, 241, 242, 243, 244, 245,
                246, 247, 248, 249, 250, 251, 252, 253, 254, 255
            ]
        );
    }

    #[test]
    fn test_int_to_base64() {
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        for val in 0..64 {
            assert_eq!(
                int_to_base64(val),
                alphabet.chars().nth(val as usize).unwrap()
            );
        }
    }

    #[test]
    fn test_bytes_to_hex() {
        assert_eq!(bytes_to_hex(&vec![2, 3, 120]), "020378");
    }

    #[test]
    fn test_bytes_to_base64() {
        assert_eq!(bytes_to_base64(&vec![2, 3, 120]), "AgN4");
    }

    #[test]
    fn test_hex_to_base64() {
        assert_eq!(hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
         "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }
}
