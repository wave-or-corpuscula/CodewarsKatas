/*Write Number in Expanded Form

You will be given a number and you will need to return it as a string in Expanded Form. For example:

   12 --> "10 + 2"
   45 --> "40 + 5"
70304 --> "70000 + 300 + 4"

NOTE: All numbers will be whole numbers greater than 0.

If you liked this kata, check out part 2!! (https://www.codewars.com/kata/write-number-in-expanded-form-part-2)
 */



fn expanded_form(n: u64) -> String {
    let mut exp_form: Vec<String> = Vec::new();

    for (i, ch) in n.to_string().chars().rev().enumerate() {
        if ch != '0' { exp_form.insert(0, format!("{}{}", ch, "0".repeat(i))); }
    }
    exp_form.join(" + ")
}

fn main() {
    let input: u64 = u64::MAX;
    let result = expanded_form(input);
    println!("result: {}", result);
}

// Interesting string interactoin::::

fn _expanded_form(n: u64) -> String {
    n.to_string()
        .chars()
        .rev()
        .zip(0..)
        .filter(|&(c, _)| c != '0')
        .map(|(c, p)| format!("{}{}", c, "0".repeat(p)))
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join(" + ")
}
