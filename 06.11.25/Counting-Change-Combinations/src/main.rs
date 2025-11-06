/*Write a function that counts how many different ways you can make change for an amount of money, 
given an array of coin denominations. 
For example, there are 3 ways to give change for 4 if you have coins with denomination 1 and 2:

1+1+1+1, 1+1+2, 2+2.

The order of coins does not matter:

1+1+2 == 2+1+1

Also, assume that you have an infinite amount of coins.

Your function should take an amount to change and an array of unique denominations for the coins:

  count_change(4, &[1, 2]) // => 3
  count_change(10, &[5, 2, 3]) // => 4
  count_change(11, &[5, 7]) //  => 0
 */

 use std::collections::HashMap;

 pub fn count_change(money: u32, coins: &[u32]) -> u64 {
    fn rec(m: u32, coins: &[u32], memo: &mut HashMap<(u32, usize), u64>) -> u64 {
        if m == 0 {
            return 1;
        }
        if m < 0 || coins.is_empty() {
            return 0;
        }
        let key = (m, coins.len());
        if let Some(&v) = memo.get(&key) {
            return v;
        }

        let use_it = if m >= coins[0] {
            rec(m - coins[0], coins, memo)
        } else {
            0
        };
        let skip_it = rec(m, &coins[1..], memo);

        let res = use_it + skip_it;
        memo.insert(key, res);
        res
    }

    rec(money, coins, &mut HashMap::new())
}

fn main() {
    println!("Hello, world!");
}
