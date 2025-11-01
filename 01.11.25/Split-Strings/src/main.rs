/*Complete the solution so that it splits the string into pairs of two characters. 
If the string contains an odd number of characters then it should replace the missing second character of the final pair with an underscore ('_').

Examples:

* 'abc' =>  ['ab', 'c_']
* 'abcdef' => ['ab', 'cd', 'ef']
*/

fn solution(s: &str) -> Vec<String> {
    let _s = {
        if s.len() % 2 == 1 { String::from(s) + "_" } else { String::from(s) }
    };
    
    let result = _s.as_bytes()
    .chunks(2)
    .map(|buf| unsafe { String::from_utf8_unchecked(buf.to_vec()) })
    .collect::<Vec<String>>();
    
    result
}


fn main() {
    println!("{:?}", solution("привет"));
}
