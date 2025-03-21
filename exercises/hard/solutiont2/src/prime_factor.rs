use std::{ops::Add, u128};

use rand::{thread_rng, Rng};

pub fn find_max_prime_factor(number: u128) -> u128 {

    let mut max_factor = 0;
    max_factor = fac(number as i128, max_factor);
    println!("max_factor={}", max_factor);
    0
}

pub fn gcd(a: i128, b: i128)->i128{
    if b == 0{
        return a;
    }
    gcd(b, a % b)
}


fn bmul( a: i128, b: i128,  m: i128)->i128 {  // 快速乘
    println!("a={}, b={},result={}, m={}", a, b,(((a as f64) / (m as f64) * (b as f64) + 0.5) as u128), m);
    // let c: u128 = (a as u128) * (b as u128) -  (((a as f64) / (m as f64) * (b as f64) + 0.5) as u128) * (m as u128);
    let c: u128 = (a as u128) * (b as u128) -  (((a as f64) / (m as f64) * (b as f64)) as u128) * (m as u128);
    println!("c={}", c);
    // ull c = (ull)a * (ull)b - (ull)((long double)a / m * b + 0.5L) * (ull)m;
    if c < m as u128 {
        return c as i128;
    }
    c as i128 + m
  }
  
  fn qpow( mut x: i128, mut p: i128, m: i128) -> i128 {  // 快速幂
    let mut ans: i128 = 1;
    while p != 0 {
      if p & 1 != 0 {
        ans = bmul(ans, x, m);
      }
      x = bmul(x, x, m);
      p >>= 1;
    }
    return ans;
  }
  
  fn Miller_Rabin(p : i128) -> bool {  // 判断素数
    if p < 2 {
        return false;
    }
    if p == 2 {
        return true;
    }
    if p == 3 {
        return true;
    }
    let mut d = p - 1;
    let mut r: i128 = 0;
    while (d & 1) == 0 {
        r += 1;
        d >>= 1; 
     } // 将d处理为奇数
     let mut k: u128 = 0;
     while k < 10{
        k += 1;
        let mut  rng = thread_rng();
        let rand: i16 = rng.gen_range(0..i16::MAX);
        let a: i128  = (rand as i128) % (p - 2) + 2;
        let mut x: i128 = qpow(a, d, p);
        if x == 1 || x == p -1{
            continue;
        }
        let mut i =0;
        while i < r - 1 {
            i += 1;
            x = bmul(x, x, p);
            if x == p - 1 {
                break;
            }
        }
        if x != p - 1 {
            return false;
        }
     }
     true
  }
  
  fn Pollard_Rho( x: i128)->i128 {
    let mut s: i128 = 0;
    let mut t: i128 = 0;
    let mut rng = thread_rng();
    let r: i16 = rng.gen_range(0..i16::MAX);
    let c: i128 = (r as i128) % (x - 1) + 1;
    let mut step: i32 = 1;
    let mut goal: i32 = 1;
    // int step = 0, goal = 1;
    let mut  val: i128 = 1;
    // ll val = 1;
    while true {
        step = 1;
        while step <= goal {
            step += 1;
            t = (bmul(t, t, x) + c) % x;
            val = bmul(val, (t - s).abs() as i128, x);
            if (step % 127) == 0 {
                let d = gcd(val, x);
                if d > 1 {
                    return d;
                }
            }
        }

        let d = gcd(val, x);
        if d > 1 {
            return d;
        }

        goal *= 2;
        s = t;
        val = 1;
    }
    0
  }
  
  fn fac( number: i128, mut max_factor: i128) -> i128 {
    let mut x = number;
    if x <= max_factor || x < 2 {
        return max_factor;
    }
    if Miller_Rabin(x) {              // 如果x为质数
      max_factor = i128::max(max_factor, x);  // 更新答案
      return max_factor;
    }
    let mut p: i128 = x;
    while p >= x {
        p = Pollard_Rho(x);  // 使用该算法
    }
    while (x % p) == 0{
        x /= p;
    } 
    max_factor = fac(x, max_factor);
    max_factor= fac(p, max_factor);  // 继续向下分解x和p
    max_factor
  }
  