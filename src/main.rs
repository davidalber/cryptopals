extern crate cryptopals;

mod set1 {
    use cryptopals::analysis::english_score;
    use cryptopals::conversions::{Base64Value, HexValue};
    use cryptopals::encrypt::repeating_key_xor;
    use std::fs::File;
    use std::io::prelude::*;

    const CIPHER_CHARS: &str = "abcdefghijklmnopqrsqtuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789`~!@#$%^&*()-_=+;:',<.>/?";

    pub fn challenge1() {
        // https://cryptopals.com/sets/1/challenges/1
        if let Ok(hex) = HexValue::from_str("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d") {
            let result = hex.to_base64();
            assert_eq!(result, Base64Value::from_str("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t").unwrap());
            println!("{} -> {}", hex, result);
        }
    }

    pub fn challenge2() {
        // https://cryptopals.com/sets/1/challenges/2
        if let Ok(hex1) = HexValue::from_str("1c0111001f010100061a024b53535009181c") {
            if let Ok(hex2) = HexValue::from_str("686974207468652062756c6c277320657965") {
                let result = hex1.xor(&hex2);
                assert_eq!(result, HexValue::from_str("746865206b696420646f6e277420706c6179").unwrap());
                println!("{} ^ {} -> {}", hex1, hex2, result);
            }
        }
    }

    fn score_single_char_xor_decrypt(encrypted_str: &str) -> (i32, String) {
        if let Ok(encrypted) = HexValue::from_str(encrypted_str) {
            let winner = CIPHER_CHARS
                .chars()
                .map(|l| {
                    let key_str: String =
                        vec![HexValue::byte_to_hex(l as u8); encrypted_str.len() / 2]
                            .into_iter()
                            .collect();
                    if let Ok(key) = HexValue::from_str(&key_str) {
                        if let Ok(decrypted) = String::from_utf8(encrypted.xor(&key).bytes) {
                            return (english_score(&decrypted), l);
                        }
                    }
                    (0, l)
                }).max_by_key(|&(s, _)| s)
                .unwrap();

            let key_str: String =
                vec![HexValue::byte_to_hex(winner.1 as u8); encrypted_str.len() / 2]
                    .into_iter()
                    .collect();
            if let Ok(key) = HexValue::from_str(&key_str) {
                if let Ok(decrypted) = String::from_utf8(encrypted.xor(&key).bytes) {
                    return (winner.0, decrypted);
                }
            }
        }
        (0, String::from(""))
    }

    pub fn challenge3() {
        // https://cryptopals.com/sets/1/challenges/3
        let encrypted_str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let (_, decrypted) = score_single_char_xor_decrypt(encrypted_str);
        println!("{:?}", decrypted);
    }

    pub fn challenge4() {
        // https://cryptopals.com/sets/1/challenges/4
        let mut f = File::open("data/4.txt").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("something went wrong reading the file");
        let mut results = Vec::new();
        for line in contents.split("\n") {
            results.push(score_single_char_xor_decrypt(line));
        }

        let winner = results.iter().max_by_key(|(s, _)| s).unwrap();
        println!("{:?}", winner.1);
    }

    pub fn challenge5() {
        // https://cryptopals.com/sets/1/challenges/4
        let msg = r"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
        println!("{}", repeating_key_xor(msg, "ICE"));
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

    println!("\n# Challenge 4");
    set1::challenge4();

    println!("\n# Challenge 5");
    set1::challenge5();
}
