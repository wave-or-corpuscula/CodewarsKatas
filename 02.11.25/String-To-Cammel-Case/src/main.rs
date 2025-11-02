/*Complete the method/function so that it converts dash/underscore delimited words into camel casing. 
The first word within the output should be capitalized only if the original word was capitalized 
(known as Upper Camel Case, also often referred to as Pascal case). The next words should be always capitalized.
Examples

"the-stealth-warrior" gets converted to "theStealthWarrior"

"The_Stealth_Warrior" gets converted to "TheStealthWarrior"

"The_Stealth-Warrior" gets converted to "TheStealthWarrior"
 */

fn to_camel_case(text: &str) -> String {
    let binding = text.replace("_", "-");
    let mut words = binding.split("-");

    format!("{}{}", words.nth(0).unwrap(), words.map(|word| capitalize_word(word)).collect::<Vec<String>>()[0..].join(""))
}

fn capitalize_word(word: &str) -> String {
    format!("{}{}", word.chars().nth(0).unwrap().to_uppercase(), word[1..].to_string())
}

fn main() {
    let text = "";
    println!("text: {}, result: {}", text, to_camel_case(text));
}


/*
Interesting use of split and map functions

fn to_camel_case(text: &str) -> String {
    text.split(&['-', '_'])
        .enumerate()
        .map(|(i, w)| match i {
            0 => w.to_string(),
            _ => w[..1].to_uppercase() + &w[1..],
        })
        .collect()
}
*/