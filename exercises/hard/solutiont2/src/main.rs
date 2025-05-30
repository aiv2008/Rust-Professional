// I AM NOT DONE

use std::cmp::Ordering;

// use rand::Rng;
use std::time::Instant;


use prime_factor::BigInt;

mod prime_factor;

use rand::Rng;

fn main() {

    let number = 199999999999999951437;
    let res = prime_factor::find_max_prime_factor(number);
    println!("{number}'s max prime factor: {res}");

    // println!("f={}", (i128::MAX as f64).sqrt())
    // let input_numbers = vec![97993999919999958437]; // 示例输入
    // let x = 1199999999951437;
    // for &x in &input_numbers {

    // }
}

