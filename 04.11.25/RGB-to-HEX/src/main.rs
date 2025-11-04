/*The rgb function is incomplete. Complete it so that passing in RGB decimal 
values will result in a hexadecimal representation being returned. 
Valid decimal values for RGB are 0 - 255. Any values that fall out of that range must be rounded to the closest valid value.

Note: Your answer should always be 6 characters long, the shorthand with 3 will not work here.
Examples (input --> output):

255, 255, 255 --> "FFFFFF"
255, 255, 300 --> "FFFFFF"
0, 0, 0       --> "000000"
148, 0, 211   --> "9400D3"
 */

fn rgb(r: i32, g: i32, b: i32) -> String {
    [r, g, b].iter().map(|&c| format!("{:02X}", c.clamp(0, 255))).collect::<String>()
}

fn main() {
    let (r, g, b) = (1, 257, 74);
    println!("For ({}, {}, {}), result is: {}", r, g, b, rgb(r, g, b));
}
