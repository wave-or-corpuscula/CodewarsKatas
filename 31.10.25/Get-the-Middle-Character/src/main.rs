#[allow(unused_variables)]

/*
You are going to be given a non-empty string. Your job is to return the middle character(s) of the string.

    If the string's length is odd, return the middle character.
    If the string's length is even, return the middle 2 characters.

Examples:

"test" --> "es"
"testing" --> "t"
"middle" --> "dd"
"A" --> "A"
*/


fn get_middle(s: &str) -> &str {
    let len = s.len();
    let mid = len / 2;

    if len % 2 == 0 { &s[mid - 1..mid + 1]} else { &s[mid..mid + 1] }
}


fn main() {
    println!("{}", get_middle("test"));     // es
    println!("{}", get_middle("testing"));  // t
    println!("{}", get_middle("middle"));   // dd
    println!("{}", get_middle("A"));        // A
}
