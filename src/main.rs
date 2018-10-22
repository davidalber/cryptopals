extern crate cryptopals;

mod set1 {
    use cryptopals::analysis::english_score;
    use cryptopals::conversions::HexValue;

    const ASCII_LOWER: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    pub fn challenge1() {
        // https://cryptopals.com/sets/1/challenges/1
        if let Ok(hex) = HexValue::from_str("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d") {
            println!("{} -> {}", hex, hex.to_base64());
        }
    }

    pub fn challenge2() {
        // https://cryptopals.com/sets/1/challenges/2
        if let Ok(hex1) = HexValue::from_str("1c0111001f010100061a024b53535009181c") {
            if let Ok(hex2) = HexValue::from_str("686974207468652062756c6c277320657965") {
                println!("{} ^ {} -> {}", hex1, hex2, hex1.xor(&hex2));
            }
        }
    }

    pub fn challenge3() {
        // https://cryptopals.com/sets/1/challenges/3
        let encrypted_str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        if let Ok(encrypted) = HexValue::from_str(encrypted_str) {
            let winner = ASCII_LOWER
                .iter()
                .map(|l| {
                    let key_str: String =
                        vec![HexValue::byte_to_hex(*l as u8); encrypted_str.len() / 2]
                            .into_iter()
                            .collect();
                    if let Ok(key) = HexValue::from_str(&key_str) {
                        let decrypted = String::from_utf8(encrypted.xor(&key).bytes).unwrap();
                        (english_score(&decrypted), l)
                    } else {
                        (0, l)
                    }
                }).max_by_key(|&(s, _)| s)
                .unwrap();

            let key_str: String =
                vec![HexValue::byte_to_hex(*winner.1 as u8); encrypted_str.len() / 2]
                    .into_iter()
                    .collect();
            if let Ok(key) = HexValue::from_str(&key_str) {
                let decrypted = String::from_utf8(encrypted.xor(&key).bytes).unwrap();
                println!("{}", decrypted);
            }
        }
    }
}

fn main() {
    println!("=== Set 1 ===");
    println!("# Challenge 1");
    set1::challenge1();

    println!("\n# Challenge 2");
    set1::challenge2();

    println!("\n# Challenge 3");
    set1::challenge3();
}
