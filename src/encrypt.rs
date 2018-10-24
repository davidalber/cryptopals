use conversions::HexValue;

pub fn repeating_key_xor(plaintext: &str, key: &str) -> HexValue {
    let key_bytes: Vec<u8> = key.as_bytes().to_vec();
    let hex_key: HexValue = HexValue::from_bytes(
        &key_bytes
            .into_iter()
            .cycle()
            .take(plaintext.len())
            .collect(),
    );

    let hex_plaintext = HexValue::from_bytes(&plaintext.as_bytes().to_vec());
    hex_plaintext.xor(&hex_key)
}

#[cfg(test)]
mod tests {
    use encrypt::*;

    #[test]
    fn test_repeating_key_xor() {
        let msg = r"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
        assert_eq!(repeating_key_xor(msg, "ICE"), HexValue::from_str("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f").unwrap());
    }
}
