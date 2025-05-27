// I AM NOT DONE

use std::cmp::Ordering;

// use rand::Rng;
use std::time::Instant;


use prime_factor::BigInt;

mod prime_factor;

fn main() {

    // let number = 10000071;
    // let res = prime_factor::find_max_prime_factor(number);
    // println!("{number}'s max prime factor: {res}");

    let input_numbers = vec![97993999919999958437]; // 示例输入

    for &x in &input_numbers {
        if miller_rabin(x) {
            println!("isprime");
            println!("{}", x);
        } else {
            println!("noprime");
            let mut factors = Vec::new();
            find_factor(x, &mut factors);
            factors.sort();
            let unique_factors: Vec<_> = factors.iter().copied().collect();
            for (i, &factor) in unique_factors.iter().enumerate() {
                if i == 0 || factor != unique_factors[i - 1] {
                    print!("{} ", factor);
                }
            }
            println!();
        }
    }
}


use rand::Rng;

fn quick_multiply(a: i128, mut b: i128, mod_val: i128) -> i128 {
    let mut ans = 0;
    let mut res = a % mod_val;
    while b > 0 {
        if b & 1 == 1 {
            ans = (ans + res) % mod_val;
        }
        res = (res + res) % mod_val;
        b >>= 1;
    }
    ans
}

fn quick_power(mut a: i128, mut b: i128, mod_val: i128) -> i128 {
    let mut ans = 1;
    a %= mod_val;
    while b > 0 {
        if b & 1 == 1 {
            ans = (ans * a) % mod_val;
        }
        a = (a * a) % mod_val;
        b >>= 1;
    }
    ans
}

fn miller_rabin(n: i128) -> bool {
    if n < 2 {
        return false;
    }
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    if primes.contains(&n) {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    for &a in &primes {
        if a >= n {
            continue;
        }
        let mut x = quick_power(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut found = false;
        for _ in 0..s {
            x = quick_multiply(x, x, n);
            if x == n - 1 {
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }
    true
}


fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a.abs()
}

fn pollard_rho(n: i128, c: i128) -> i128 {
    if n % 2 == 0 {
        return 2;
    }
    if n % 3 == 0 {
        return 3;
    }
    if n % 5 == 0 {
        return 5;
    }

    let mut rng = rand::thread_rng();
    let mut x = rng.gen_range(0..n);
    let mut y = x;
    let mut d = 1;
    let mut offset = c;

    while d == 1 {
        x = (quick_multiply(x, x, n) + offset) % n;
        y = (quick_multiply(y, y, n) + offset) % n;
        y = (quick_multiply(y, y, n) + offset) % n;
        d = gcd((x - y).abs(), n);
        if x == y {
            offset += 1;
            x = 0;
            y = 0;
        }
    }
    d
}

fn find_factor(n: i128, factors: &mut Vec<i128>) {
    if miller_rabin(n) {
        factors.push(n);
        return;
    }
    let divisor = pollard_rho(n, 1);
    find_factor(divisor, factors);
    find_factor(n / divisor, factors);
}
