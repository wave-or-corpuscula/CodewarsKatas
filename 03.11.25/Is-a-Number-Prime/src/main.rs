/*Define a function that takes an integer argument and returns a logical value true or false depending on if the integer is a prime.

Per Wikipedia, a prime number ( or a prime ) is a natural number greater than 1 that has no positive divisors other than 1 and itself.
Requirements

    You can assume you will be given an integer input.
    You can not assume that the integer will be only positive. You may be given negative numbers as well ( or 0 ).
    NOTE on performance: There are no fancy optimizations required, but still the most trivial solutions might time out. 
    Numbers go up to 2^31 ( or similar, depending on language ). Looping all the way up to n, or n/2, will be too slow.
 */


fn is_prime_beauty_but_slow(x: i64) -> bool {
    if x <= 1 {
        return false
    }
    (2..=x/2).all(|i| x % i != 0)
}

fn is_prime(x: i64) -> bool {
    if x <= 1 {
        return false;
    }
    if x == 2 || x == 3 {
        return true;
    }

    let mut i = 5;
    while i * i < x {
        if (x % i == 0) || (x % (i + 2) == 0) {
            return false
        }
        i += 6;
    }
    true
}


// Красивое, но не мое
fn is_prime_from_code_wars(x: i64) -> bool {
    let last = (x as f64).sqrt() as i64 + 1;
    x > 1 && (2..last).all(|d| x % d != 0)
}


fn main() {
    println!("{}", is_prime(4));
}
