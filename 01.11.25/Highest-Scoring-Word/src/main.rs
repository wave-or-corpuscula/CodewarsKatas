/*Given a string of words, you need to find the highest scoring word.

Each letter of a word scores points according to its position in the alphabet: a = 1, b = 2, c = 3 etc.

For example, the score of abad is 8 (1 + 2 + 1 + 4).

You need to return the highest scoring word as a string.

If two words score the same, return the word that appears earliest in the original string.

All letters will be lowercase and all inputs will be valid.
 */

fn high(input: &str) -> &str {
    let (mut max_sum, mut max_ind) = (0, 0);
    let words: Vec<&str> = input.split(" ").collect();
    let zero_value: u8 = 'a' as u8 - 1;

    for (i, word) in words.iter().enumerate() {
        let word_sum = word.chars().map(|c| ((c as u8) - zero_value) as i32).sum();
        if word_sum > max_sum {
            max_sum = word_sum;
            max_ind = i;
        }
    }
    words[max_ind]
}

fn main() {
    let word = "aa b";

    println!("With word: \'{}\', result is: {}", word, high(word));
}
