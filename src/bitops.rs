use conversions::{bytes_to_hex, hex_to_bytes};

pub fn hex_xor(hex1: &str, hex2: &str) -> String {
    assert!(hex1.len() == hex2.len());
    let bytes1 = hex_to_bytes(hex1);
    let bytes2 = hex_to_bytes(hex2);
    let mut result = Vec::with_capacity(bytes1.len());

    for (byte1, byte2) in bytes1.iter().zip(bytes2.iter()) {
        result.push(byte1 ^ byte2);
    }

    bytes_to_hex(&result)
}

#[cfg(test)]
mod tests {
    use bitops::*;

    #[test]
    fn test_hex_xor() {
        assert_eq!(
            hex_xor(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965"
            ),
            "746865206b696420646f6e277420706c6179"
        );
    }

    #[test]
    #[should_panic]
    fn test_hex_xor_different_lengths() {
        hex_xor(
            "1c0111001f010100061a024b53535009181c",
            "686974207468652062756c6c2773206579",
        );
    }
}
