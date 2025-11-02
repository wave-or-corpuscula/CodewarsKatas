/*This time we want to write calculations using functions and get the results. Let's have a look at some examples:

seven!(times(five!()))   // must return 35
four!(plus(nine!()))     // must return 13
eight!(minus(three!()))  // must return 5
six!(divided_by(two!())) // must return 3

Requirements:

    There must be a macro for each number from 0 ("zero") to 9 ("nine"). We use macros instead of functions for numbers, 
    because macros help to deal with lack of optional arguments in functions.
    There must be a function for each of the following mathematical operations: plus, minus, times, divided_by
    Each calculation consist of exactly one operation and two numbers
    The most outer function represents the left operand, the most inner function represents the right operand
    Division should be integer division. For example, this should return 2, not 2.666666...:

eight!(divided_by(three!()));

 */

// macro_rules! zero  { () => {0}; ($x:expr) => {{ let (op,val)=$x; op(0,val) }}; }
// macro_rules! one   { () => {1}; ($x:expr) => {{ let (op,val)=$x; op(1,val) }}; }
// macro_rules! two   { () => {2}; ($x:expr) => {{ let (op,val)=$x; op(2,val) }}; }
// macro_rules! three { () => {3}; ($x:expr) => {{ let (op,val)=$x; op(3,val) }}; }
// macro_rules! four  { () => {4}; ($x:expr) => {{ let (op,val)=$x; op(4,val) }}; }
// macro_rules! five  { () => {5}; ($x:expr) => {{ let (op,val)=$x; op(5,val) }}; }
// macro_rules! six   { () => {6}; ($x:expr) => {{ let (op,val)=$x; op(6,val) }}; }
// macro_rules! seven { () => {7}; ($x:expr) => {{ let (op,val)=$x; op(7,val) }}; }
// macro_rules! eight { () => {8}; ($x:expr) => {{ let (op,val)=$x; op(8,val) }}; }
// macro_rules! nine  { () => {9}; ($x:expr) => {{ let (op,val)=$x; op(9,val) }}; }

// type T = (fn(i32, i32) -> i32, i32);

// pub fn plus(y: i32) -> T { ((|a, b| a + b), y) }
// pub fn minus(y: i32) -> T { ((|a, b| a - b), y) }
// pub fn times(y: i32) -> T { ((|a, b| a * b), y) }
// pub fn divided_by(y: i32) -> T { ((|a, b| a / b), y) }
// По какой-то причине эта красота не захотела работать(((


pub fn plus(y: i32) -> impl Fn(i32) -> i32 {
    move |x| x + y
}

pub fn minus(y: i32) -> impl Fn(i32) -> i32 {
    move |x| x - y
}

pub fn times(y: i32) -> impl Fn(i32) -> i32 {
    move |x| x * y
}

pub fn divided_by(y: i32) -> impl Fn(i32) -> i32 {
    move |x| x / y
}

macro_rules! num_macro {
    ($name:ident, $value:expr) => {
        macro_rules! $name {
            () => {
                $value
            };
            ($x:expr) => {
                $x($value)
            };
        }
    };
}

num_macro!(zero, 0);
num_macro!(one, 1);
num_macro!(two, 2);
num_macro!(three, 3);
num_macro!(four, 4);
num_macro!(five, 5);
num_macro!(six, 6);
num_macro!(seven, 7);
num_macro!(eight, 8);
num_macro!(nine, 9);


fn main() {
    println!("{}", seven!(times(five!())));       // 35
    println!("{}", four!(plus(nine!())));         // 13
    println!("{}", eight!(minus(three!())));      // 5
    println!("{}", six!(divided_by(two!())));     // 3
    println!("{}", eight!(divided_by(three!()))); // 2
}
