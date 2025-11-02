/*Write an algorithm that will identify valid IPv4 addresses in dot-decimal format. IPs should be considered valid if they consist of four octets, with values between 0 and 255, inclusive.
Valid inputs examples:

Examples of valid inputs:
1.2.3.4
123.45.67.89

Invalid input examples:

1.2.3
1.2.3.4.5
123.456.78.90
123.045.067.089

Notes:

    Leading zeros (e.g. 01.02.03.04) are considered invalid
    Inputs are guaranteed to be a single string
 */

 use std::net::Ipv4Addr;

// fn is_valid_ip(ip: &str) -> bool {
//     let octets: Vec<&str> = ip.split(".").collect();
//     if octets.len() != 4 {
//         return false
//     }
//     octets.iter().all(|octet| is_valid_octet(octet))
// }

// fn is_valid_octet(octet: &str) -> bool {
//     if !octet.chars().all(|dig| dig.is_digit(10)) {
//         return false
//     }
//     let octet_int: i32 = octet.parse().unwrap_or(0);
//     if octet.chars().nth(0).unwrap_or('0') == '0' && octet.chars().count() != 1 {
//         return false
//     }
//     if octet_int < 0 || octet_int > 255 {
//             return false
//         }
//     true
// }

fn is_valid_ip(ip: &str) -> bool {
    ip.parse::<Ipv4Addr>().is_ok()
}


fn main() {
    let ip = "192.168.0.1";
    println!("Current ip: {}, validnes: {}", ip, is_valid_ip(ip));
}
