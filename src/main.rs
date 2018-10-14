extern crate cryptopals;

mod set1 {
    use cryptopals::bitops::hex_xor;
    use cryptopals::conversions::hex_to_base64;

    pub fn challenge1() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        println!("{} -> {}", hex, hex_to_base64(hex));
    }

    pub fn challenge2() {
        let hex1 = "1c0111001f010100061a024b53535009181c";
        let hex2 = "686974207468652062756c6c277320657965";
        println!("{} ^ {} -> {}", hex1, hex2, hex_xor(hex1, hex2));
    }
}

fn main() {
    println!("=== Set 1 ===");
    println!("# Challenge 1");
    set1::challenge1();

    println!("\n# Challenge 2");
    set1::challenge2();
}
