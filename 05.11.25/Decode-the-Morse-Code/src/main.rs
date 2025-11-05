use std::collections::HashMap;


fn decode_morse(encoded: &str) -> String {

    // let mut MORSE_CODE: HashMap<String, String> = HashMap::new();
    // MORSE_CODE.insert(".-".to_string(), "A".to_string());
    // MORSE_CODE.insert("-...".to_string(), "B".to_string());
    // MORSE_CODE.insert("-.-.".to_string(), "C".to_string());

    encoded
        .trim()
        .split("   ")
        .map(|word| {
            word.split(' ')
                .filter(|c| !c.is_empty())
                .map(|code| MORSE_CODE[code].as_str())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}


fn main() {

    let encoded = ".- .-   -... -... -...   .- -.-. -.-. ";

    println!("result: {}", decode_morse(encoded));
}
