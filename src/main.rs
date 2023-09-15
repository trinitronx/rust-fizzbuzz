/*! # FizzBuzz Code Golf

Consider the following problem:

Write a short program that prints each number from 1 to 100 on a new line.
 - For each multiple of 3, print "`Fizz`" instead of the number.
 - For each multiple of 5, print "`Buzz`" instead of the number.
 - For numbers which are multiples of both 3 and 5, print "`FizzBuzz`" instead of the number.
 - Write a solution (or reduce an existing one) so it has as few characters as possible.

Scoring
- Your score is: `200 - number of characters in your source code) / 100`

Source: [HackerRank: FizzBuzz][1]

[1]: https://www.hackerrank.com/challenges/fizzbuzz/problem
*/

/**
# Rust `FizzBuzz` in 125 chars

This is hideous.  Normally we would never want or need such minified code,
except in the [silly][1] competition known as "Code Golf".

Rust apparently allows for removing whitespace from code, to minify it in a
similar way as JavaScript, another language sometimes similar to BrainFuck
which allows us to write hideously minified code.

Running `rustfmt` on this suffers the whitespace penalty, but does make it more
readable.

[1]:
    https://web.archive.org/web/20230914220048if_/https://i.pinimg.com/736x/b4/fb/cd/b4fbcdd779d811a2b2493a0d7b3929a2--fantasy-quotes-monty-python.jpg
*/
fn main(){for i in 1..101{let f=i%3<1;let b=i%5<1;if f{print!("Fizz")}if b{print!("Buzz")}if!(f|b){print!("{i}")}println!()}}

/// # An even smaller `main()` function
///
/// Using an import alias and macro, we can slim down the `main()` function
/// further.  Yet, the boilerplate syntax actually makes the total number of
/// characters in the code larger.
use std::print as p;
use std::println as n;
macro_rules! l{($b:tt,$i:tt,$x:tt)=>{let$b=$i%$x<1;};}
// Smallest main() function: 103 chars
fn pain(){for i in 1..101{l!(f,i,3);l!(b,i,5);if f{p!("Fizz")}if b{p!("Buzz")}if!(f|b){p!("{i}")}n!()}}