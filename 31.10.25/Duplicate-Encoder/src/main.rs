/*
The goal of this exercise is to convert a string to a new string where each character in the new string 
is "(" if that character appears only once in the original string, or ")" if that character appears more than once in the original string. Ignore capitalization when determining if a character is a duplicate.
Examples

"din"      =>  "((("
"recede"   =>  "()()()"
"Success"  =>  ")())())"
"(( @"     =>  "))((" 
*/


use std::collections::HashMap;


fn duplicate_encode(word:&str)->String {
    let mut counts: HashMap<char, i32> = HashMap::new();
    let mut result: String = String::new();

    for ch in word.chars().map(|c| c.to_ascii_lowercase()) {
        *counts.entry(ch).or_insert(0) += 1;
    }
    for ch in word.chars().map(|c| c.to_ascii_lowercase()) {
        if counts[&ch] > 1 { // *counts.get(&ch).unwrap() > 1
            result.push(')');
        } else {
            result.push('(');
        }
    }
    result
}

fn main() {
    println!("{}", duplicate_encode("din"));
    println!("{}", duplicate_encode("recede"));
    println!("{}", duplicate_encode("Success"));
    println!("{}", duplicate_encode("(( @"));
}
