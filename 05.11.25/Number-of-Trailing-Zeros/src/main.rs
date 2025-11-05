/*Write a program that will calculate the number of trailing zeros in a factorial of a given number.

N! = 1 * 2 * 3 *  ... * N

Be careful 1000! has 2568 digits...

For more info, see: http://mathworld.wolfram.com/Factorial.html
Examples
N 	Product 	N factorial 	Trailing zeros
6 	1*2*3*4*5*6 	720 	1
12 	1*2*3*4*5*6*7*8*9*10*11*12 	479001600 	2 */


fn zeros(n: u64) -> u64 {
    let (mut k, mut sum) = (n, 0);

    while k >= 5 {
        k /= 5;
        sum += k;
    }
    sum
}

fn zeros_recurs(n: u64) -> u64 {
    if n == 0 { 0 }
    else { n / 5 + zeros_recurs(n / 5) }
}

fn main() {
    let n = 630;
    println!("n = {n}, zeros = {}", zeros_recurs(n));

    for i in 1..=n {
        print!("{i}\t");
        if i % 10 == 0 {
            println!("|{}", i * 2 / 10);
        }
    }
    println!()
}
