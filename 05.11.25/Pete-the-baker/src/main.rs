/*Pete likes to bake some cakes. He has some recipes and ingredients. Unfortunately he is not good in maths. Can you help him to find out, how many cakes he could bake considering his recipes?

Write a function cakes(), which takes the recipe (HashMap<&str, u32>) and the available ingredients (also a HashMap<&str, u32>) and returns the maximum number of cakes Pete can bake (u32). For simplicity there are no units for the amounts (e.g. 1 lb of flour or 200 g of sugar are simply 1 or 200). Ingredients that are not present in the objects, can be considered as 0.

Examples:

// must return 2
cakes(HashMap::from([("flour", 500), ("sugar", 200), ("eggs", 1)]), HashMap::from([("flour", 1200), ("sugar", 1200), ("eggs", 5), ("milk", 200)]))
// must return 0
cakes(HashMap::from([("apples", 3), ("flour", 300), ("sugar", 150), ("milk", 100), ("oil", 100)]), HashMap::from([("sugar", 500),("flour", 2000), ("milk", 2000)]))

 */

use std::collections::HashMap;

fn cakes(recipe: &HashMap<&str, u32>, available: &HashMap<&str, u32>) -> u32 {
    let mut cakes_amount: Vec<u32> = Vec::new();
    for (ingridient, required_amount) in recipe {
        let avaliable_amount = *available.get(ingridient).unwrap_or(&0u32);
        if avaliable_amount < *required_amount { return 0 }
        cakes_amount.push(avaliable_amount / required_amount);
    }
    *cakes_amount.iter().min().unwrap()
}

fn cakes_elegant(recipe: &HashMap<&str, u32>, available: &HashMap<&str, u32>) -> u32 {
    recipe
    .iter()
    .map(|(key, value)| available.get(key).unwrap_or(&0) / value)
    .min()
    .unwrap_or(0)
}

macro_rules! map {
    () => { HashMap::new() };
    ($($ingredient:ident : $amount:expr),+) => {{
        let mut map = HashMap::new();
        $( map.insert(stringify!($ingredient), $amount); )*
        map
    }};
}

fn main() {
    let recipe = map!(flour: 500, sugar: 200, eggs: 1);
    let available = map!(flour: 400, sugar: 1200, eggs: 5, milk: 200);
    println!("Number of cakes: {}", cakes(&recipe, &available));
}
