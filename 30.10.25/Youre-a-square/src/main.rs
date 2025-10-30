/*

A square of squares

You like building blocks. You especially like building blocks that are squares. And what you even like more, is to arrange them into a square of square building blocks!

However, sometimes, you can't arrange them into a square. Instead, you end up with an ordinary rectangle! Those blasted things! If you just had a way to know, whether you're currently working in vain… Wait! That's it! You just have to check if your number of building blocks is a perfect square.
Task

Given an integral number, determine if it's a square number:

*/


// fn is_square(n: i64) -> bool {
//     let root: f64 = (n as f64).sqrt() as f64;
//     return (root - (root as i32) as f64) == 0.0;
// }

fn is_square(n: i64) -> bool {
    (n as f64).sqrt().fract() == 0.0
}


fn main() {
    println!("Result: {}", is_square(4));
}
