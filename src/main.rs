extern crate cryptopals;

mod set1 {
    use cryptopals::analysis::english_score;
    use cryptopals::bitops::hex_xor;
    use cryptopals::conversions::{ byte_to_hex, hex_to_bytes, hex_to_base64 };

    const ASCII_LOWER: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
                                    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
                                    's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

    pub fn challenge1() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        println!("{} -> {}", hex, hex_to_base64(hex));
    }

    pub fn challenge2() {
        let hex1 = "1c0111001f010100061a024b53535009181c";
        let hex2 = "686974207468652062756c6c277320657965";
        println!("{} ^ {} -> {}", hex1, hex2, hex_xor(hex1, hex2));
    }

    pub fn challenge3() {
        let encrypted = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

        let winner = ASCII_LOWER.iter().map(|l| {
            let key: String = vec![byte_to_hex(*l as u8); encrypted.len()/2].join("");
            let decrypted_bytes = hex_to_bytes(&hex_xor(encrypted, &key));
            let decrypted = String::from_utf8(decrypted_bytes).unwrap();
            (english_score(&decrypted), l)
        }).max_by_key(|&(s, _)| s).unwrap();

        let key: String = vec![byte_to_hex(*winner.1 as u8); encrypted.len()/2].join("");
        let decrypted_bytes = hex_to_bytes(&hex_xor(encrypted, &key));
        println!("{}", String::from_utf8(decrypted_bytes).unwrap());
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
