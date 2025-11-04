/*he number 898989 is the first integer with more than one digit that fulfills the property 
partially introduced in the title of this kata. What's the use of saying "Eureka"? 
Because this sum gives the same number: 89 = 8^1 + 9^2

The next number in having this property is 135135135:

See this property again: 135=11+32+53135 = 1^1 + 3^2 + 5^3135=11+32+53 */




fn sum_dig_pow(a: u64, b: u64) -> Vec<u64> {
    (a..=b)
        .filter(|&num| {
            let sum = num
                .to_string()
                .chars()
                .enumerate()
                .map(|(i, d)| (d.to_digit(10).unwrap() as u64).pow((i as u32) + 1))
                .sum::<u64>();
            sum == num
        })
        .collect()
}


fn main() {
    // let var = u64::MAX * 2;
    let (a, b) = (u64::MAX - 20, u64::MAX - 10);
    println!("For bounds: ({}, {}), result: {:?}", a, b , sum_dig_pow(a, b));
}
