/*You live in the city of Cartesia where all roads are laid out in a perfect grid. You arrived ten minutes too early to an appointment, so you decided to take the opportunity to go for a short walk. The city provides its citizens with a Walk Generating App on their phones -- everytime you press the button it sends you an array of one-letter strings representing directions to walk (eg. ['n', 's', 'w', 'e']). You always walk only a single block for each letter (direction) and you know it takes you one minute to traverse one city block, so create a function that will return true if the walk the app gives you will take you exactly ten minutes (you don't want to be early or late!) and will, of course, return you to your starting point. Return false otherwise.

    Note: you will always receive a valid array containing a random assortment of direction letters ('n', 's', 'e', or 'w' only). It will never give you an empty array (that's not a walk, that's standing still!).

 */

use std::collections::HashMap;

fn is_valid_walk(walk: &[char]) -> bool {
    if walk.len() != 10 {
        return false
    }
    let els_freq = walk
    .iter()
    .copied()
    .fold(HashMap::new(), |mut map, val|{
        map.entry(val)
        .and_modify(|f| *f += 1)
        .or_insert(1);
        map
    });
    els_freq.get(&'n').unwrap_or(&0) == els_freq.get(&'s').unwrap_or(&0) &&
    els_freq.get(&'w').unwrap_or(&0) == els_freq.get(&'e').unwrap_or(&0)
}

fn main() {
    let walk = ['n','n','n','s','n','s','n','s','n','s'];
    println!("result: {}", is_valid_walk(&walk));
}


fn is_valid_walk_from_CodeWars(walk: &[char]) -> bool {
    if walk.len() != 10 { return false; }
    let (mut n_count,mut e_count) = (0,0);
    for w in walk{
        match w {
            'n' => {n_count+=1}
            'e' => {e_count+=1}
            's' => {n_count-=1}
            'w' => {e_count-=1}
            _ => {}
        }
    }
    n_count == 0 && e_count == 0
}