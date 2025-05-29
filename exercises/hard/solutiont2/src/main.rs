// I AM NOT DONE

use std::cmp::Ordering;

// use rand::Rng;
use std::time::Instant;


use prime_factor::BigInt;

mod prime_factor;

use rand::Rng;

fn main() {

    let number = 10000071;
    let res = prime_factor::find_max_prime_factor(number);
    println!("{number}'s max prime factor: {res}");

    let f = 199999999999999951437f64;
    println!("f={}", f.sqrt())
    // let input_numbers = vec![97993999919999958437]; // 示例输入
    // let x = 1199999999951437;
    // for &x in &input_numbers {

    // }
}

