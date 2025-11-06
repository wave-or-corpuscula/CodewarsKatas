/*This is the first part. You can solve the second part here when you are done with this. Multiply two numbers! Simple!

    The arguments are passed as strings.
    The numbers may be way very large
    Answer should be returned as a string
    The returned "number" should not start with zeros e.g. 0123 is invalid

Note: 100 randomly generated tests!
Important

Usage of num::BigInt is disallowed and will be checked in the full test suite.

https://www.codewars.com/kata/55911ef14065454c75000062/train/rust
 */

use std::fmt::format;

fn multiply(a: &str, b: &str) -> String {
    let mut result = String::new();
    let a_vec: Vec<u32> = a.trim_start_matches('0').chars().map(|c| c.to_digit(10).unwrap()).collect();
    let b_vec: Vec<u32> = b.trim_start_matches('0').chars().map(|c| c.to_digit(10).unwrap()).collect();

    println!("{:?}, {:?}", a_vec, b_vec);

    let (long, short) = if a_vec.len() > b_vec.len() {(a_vec, b_vec)} else {(b_vec, a_vec)};
    let mut buf = 0;

    for mult1 in short {

        for mult2 in long {
            println!("{mult1} * {mult2} = {}, buf = {}", (mult1 * mult2 + buf) % 10, (mult1 * mult2 + buf) / 10);
            buf = (mult1 * mult2 + buf) / 10;
        }
        
        println!("{var1} * {var2} = {}, buf = {}", (var1 * var2 + buf) % 10, (var1 * var2 + buf) / 10 );
        result = format!("{}{}", (var1 * var2 + buf) % 10, result);
        buf = (var1 * var2 + buf) / 10;
    }

    result
}

fn main() {
    let (a, b) = ("1009", "033");
    println!("{a} * {b} = {}", multiply(a, b));
}
