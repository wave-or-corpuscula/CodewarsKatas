/*ROT13 is a simple letter substitution cipher that replaces a letter with the letter 13 letters after it in the alphabet. 
ROT13 is an example of the Caesar cipher.

Create a function that takes a string and returns the string ciphered with Rot13. 
If there are numbers or special characters included in the string, they should be returned as they are. 
Only letters from the latin/english alphabet should be shifted, like in the original Rot13 "implementation".
 */


fn rot13(message: &str) -> String {
    message.chars().map(|ch| char_to_rot13(ch as u8)).collect::<String>()
}

fn char_to_rot13(ch: u8) -> char {
    if (ch >= 65 && ch <= 77) || (ch >= 97 && ch <= 109) { return (ch + 13) as char }
    else if (ch >= 78 && ch <= 90) || (ch >= 110 && ch <= 122) { return (ch - 13) as char }
    else { return ch as char }
}


fn main() {
    // let message = "Some message to translate^^&&**(%";
    let message = "Fbzr zrffntr gb genafyngr^^&&**(%";
    println!("{}", rot13_from_CodeWars(message));
}

fn rot13_from_CodeWars(message: &str) -> String {
    message.chars().map(|c| {
        match c {
            'A'..='M' | 'a'..='m' => ((c as u8) + 13) as char,
            'N'..='Z' | 'n'..='z' => ((c as u8) - 13) as char,
            _ => c,
        }
    }).collect()
}