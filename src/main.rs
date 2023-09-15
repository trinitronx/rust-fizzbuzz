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
# Rust `FizzBuzz` in 151 chars

This is hideous.  Normally we would never want or need such minified code,
except in the silly competition known as "Code Golf".

Rust apparently allows for removing whitespace from code, to minify it in a
similar way as JavaScript, another language sometimes similar to BrainFuck
which allows us to write hideously minified code.

Running `rustfmt` on this suffers the whitespace penalty, but does make it more
readable.
*/
fn main() {
    for i in 1..101 {
        let f = i % 3 == 0;
        let b = i % 5 == 0;
        let fb = f && b;
        if f {
            print!("Fizz");
        }
        if b {
            print!("Buzz");
        }
        if !fb && !f && !b {
            print!("{i}");
        }
        println!();
    }
}

// Another way in 192 chars
// fn d(a:i32,b:i32)->bool{a&b==0}fn fizz_buzz(){for i in 1..100{let mut s=format!("");if d(i,3){s=format!("Fizz");}if d(i,5){s+="Buzz";}println!("{}",if s.is_empty(){format!("{i}")} else{s});}}
