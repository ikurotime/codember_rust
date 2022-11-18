use std::fs;

const ASCII_Z: u8 = 122;
const ASCII_A: u8 = 97;

fn main() {
    const FILE: &str = "../encrypted.txt";
    let coded_message = fs::read_to_string(FILE).expect("No se pudo leer el archivo encrypted.txt");

    let decoded: Vec<String> = coded_message
        .split(" ")
        .map(|word| decode_word(word))
        .collect();

    println!("🔐 Coded message ->  {}", coded_message);
    println!("🔑 Decoded message -> {}", decoded.join(" "));
}

fn decode_word(word: &str) -> String {
    let arr_codes = word.as_bytes().to_vec();
    let mut decoded_message = String::from("");
    let mut new_word: String = String::from("");
    for l in arr_codes {
        new_word.push(l as char);
        let ascii_value: u8 = new_word.parse().expect("something went wrong!");
        if ascii_value <= ASCII_Z && ascii_value >= ASCII_A {
            decoded_message.push(ascii_value as char);
            new_word = String::from("");
        }
    }

    decoded_message
}
