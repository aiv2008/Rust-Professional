// use std::{ops::Add, u128};

use rand::{thread_rng, Rng};

use std::cmp::Ordering;
use std::ops::{Add, BitAnd, Div, Mul, Neg, Rem, Shr, Sub};
use crate::prime_factor;
// use fraction::Fraction;

pub fn find_max_prime_factor(number: u128) -> u128 {

    let mut max_factor = 0;
    // let result =  fac(BigInt::from(number.to_string()) ,BigInt::from(max_factor.to_string()) );
    // println!("result={:?}", result);

    if miller_rabin(number as i128) {
        println!("isprime");
        println!("{}", number);
        max_factor = number;
    } else {
        println!("noprime");
        let mut factors = Vec::new();
        find_factor(number as i128, &mut factors);
        factors.sort();
        let unique_factors: Vec<_> = factors.iter().copied().collect();
        for (i, &factor) in unique_factors.iter().enumerate() {
            if i == 0 || factor != unique_factors[i - 1] {
                max_factor = factor as u128;
                print!("{} ", factor);
            }
        }
        println!();
    }
    max_factor
    // 0
}

const BASE: i64 = 10;

// 定义一个 BigInt 结构体来表示大整数。这个结构体包含一个 Vec<i64> 类型的字段 digits，用于存储大整数的各个位数。从低位到高位动态存储。
#[derive(Debug, Clone)]
pub struct BigInt {
    digits: Vec<i64>,
    sign: bool, // false: positive, true: negative
}

impl BigInt {
    pub fn new(mut digits: Vec<i64>, sign: bool) -> Self {
        while digits.len() > 1 && digits.last() == Some(&0) {
            digits.pop();
        }
        BigInt { digits, sign }
    }

    pub fn to_string(&self) -> String{
        let sign = if self.sign { "-" } else { "" };
        let digits = self.digits.iter().rev().map(|&x| x.to_string()).collect::<Vec<_>>().join("");
        format!("{}{}", sign, digits)
    }
}

// 实现基本运算
// 比较运算，PartialEq、Eq、PartialOrd 和 Ord 比较大整数。
impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        (self.digits == other.digits && self.sign == other.sign) ||
        (self.is_zero() && other.is_zero())
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for BigInt {}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.sign != other.sign {
            return Some(if self.sign { Ordering::Less } else { Ordering::Greater });
        }
        if self.digits.len() != other.digits.len() {
            return Some(self.digits.len().cmp(&other.digits.len()));
        }
        Some(self.cmp(other))
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.sign != other.sign {
            return if self.sign { Ordering::Less } else { Ordering::Greater };
        }
        let len_cmp = self.digits.len().cmp(&other.digits.len());
        if len_cmp != Ordering::Equal {
            return len_cmp;
        }
        for (a, b) in self.digits.iter().rev().zip(other.digits.iter().rev()) {
            let cmp = a.cmp(b);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }
        Ordering::Equal
    }
}

// 从String类型转换为BigInt。
impl From<String> for BigInt {
    fn from(mut s: String) -> Self {
        let mut digits = Vec::new();
        // remove zero
        while s.starts_with('0') && s.len() > 1 {
            s.remove(0);
        }
        for c in s.chars() {
            digits.push(c.to_digit(10).unwrap() as i64);
        }
        digits.reverse();
        BigInt { digits, sign: false }
    }
}

// 相反数运算
impl Neg for BigInt {
    type Output = Self;
    fn neg(self) -> Self {
        BigInt { digits: self.digits, sign: !self.sign }
    }
}

// 加法运算
impl Add for BigInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.sign != other.sign {
            if self.digits == other.digits {
                return BigInt::zero();
            }
            return self - (-other);
        }
        let mut result = Vec::new();
        let mut carry = 0;
        let max_len = self.digits.len().max(other.digits.len());

        for i in 0..max_len {
            let a = *self.digits.get(i).unwrap_or(&0);
            let b = *other.digits.get(i).unwrap_or(&0);
            let sum = a + b + carry;
            result.push(sum % BASE);
            carry = sum / BASE;
        }

        if carry > 0 {
            result.push(carry);
        }

        while result.len() > 1 && result.last() == Some(&0) {
            result.pop();
        }

        BigInt { digits: result , sign: self.sign }
    }
}

// 减法运算
impl Sub for BigInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.sign == other.sign && self.digits == other.digits {
            return BigInt::zero();
        }
        if self.sign != other.sign {
            return self + (-other);
        }
        let mut result = Vec::new();
        let mut borrow = 0;
        let mut sign = self.sign;
        let (larger, smaller) = if self >= other {
            (self, other)
        } else {
            sign = !self.sign;
            (other, self)
        };

        for i in 0..larger.digits.len() {
            let a = *larger.digits.get(i).unwrap_or(&0);
            let b = *smaller.digits.get(i).unwrap_or(&0);
            let mut diff = a - b - borrow;
            if diff < 0 {
                diff += BASE;
                borrow = 1;
            } else {
                borrow = 0;
            }
            result.push(diff);
        }

        while result.len() > 1 && result.last() == Some(&0) {
            result.pop();
        }

        BigInt { digits: result, sign }
    }
}

// 乘法运算
impl Mul for BigInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = vec![0; self.digits.len() + other.digits.len()];

        for (i, &a) in self.digits.iter().enumerate() {
            let mut carry = 0;
            for (j, &b) in other.digits.iter().enumerate() {
            let sum = result[i + j] + a * b + carry;
            result[i + j] = sum % BASE;
            carry = sum / BASE;
            }
            result[i + other.digits.len()] += carry;
        }

        while result.len() > 1 && result.last() == Some(&0) {
            result.pop();
        }

        BigInt { digits: result, sign: self.sign ^ other.sign }
    }
}

// 除法运算
impl Div for BigInt {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if other.digits.is_empty() || other.is_zero() {
            eprintln!("Warning: Division by zero");
            return BigInt::zero();
        }
        let mut remainder = self.clone().abs();
        let mut quotient = BigInt { digits: vec![], sign: self.sign ^ other.sign };
        let mut divisor = other.abs();
        if remainder < divisor {
            return BigInt { digits: vec![0], sign: self.sign };
        }
        let divisor_len = divisor.digits.len();
        while divisor.digits.len() < remainder.digits.len() {
            divisor.digits.insert(0, 0);
        }
        while divisor.digits.len() >= divisor_len {
            let mut quotient_digit = 0;
            while remainder >= divisor {
                remainder = remainder - divisor.clone();
                quotient_digit += 1;
            }
            quotient.digits.insert(0, quotient_digit);
            divisor.digits.remove(0);
        }
        
        while quotient.digits.len() > 1 && quotient.digits.last() == Some(&0) {
            quotient.digits.pop();
        }

        quotient
    }
}

// 取余运算
impl Rem for BigInt {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        if other.digits.is_empty() || other.is_zero() {
            eprintln!("Warning: Division by zero");
            return BigInt::zero();
        }
        if other.sign {
            eprintln!("Warning: The moduli cannot be negative!");
            return BigInt::zero();
        }
        let mut remainder = self.clone().abs();
        let mut divisor = other.clone().abs();
        if remainder < divisor {
            if self.sign && !remainder.is_zero() {
                remainder = other.abs() - remainder;
            }
            return remainder;
        }
        let divisor_len = divisor.digits.len();
        while divisor.digits.len() < remainder.digits.len() {
            divisor.digits.insert(0, 0);
        }
        while divisor.digits.len() >= divisor_len {
            while remainder >= divisor {
                remainder = remainder - divisor.clone();
            }
            divisor.digits.remove(0);
        }

        if self.sign && !remainder.is_zero() {
            remainder = other.abs() - remainder;
        }
        remainder
    }
}

// 其他
// 还为BigInt实现了一些其他有用的方法，例如幂运算、模幂运算、判断是否为零、判断是否为一、判断是否为负数、计算最大公约数（方便约分）、取绝对值和阶乘等等。
impl BigInt {
    pub fn zero() -> Self {
        BigInt { digits: vec![0], sign: false }
    }
    pub fn one() -> Self {
        BigInt { digits: vec![1], sign: false }
    }
}


//&运算
impl BitAnd for BigInt {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self{
            digits: self.digits.iter().zip(rhs.digits.iter()).map(|(x, y)| *x & *y).collect(),
            sign: self.sign
        }
        
    }
}

// >>运算
impl Shr for BigInt{
    type Output = Self;
    fn shr(self, rhs: Self) -> Self::Output {
        Self{
            digits: self.digits.iter().zip(rhs.digits.iter()).map(|(x, y)| *x >> *y).collect(),
            sign: self.sign
        }
    }
}
    


impl BigInt {
    // pub fn fraction(self, other: Self) -> Fraction {
    //     if other.digits.is_empty() || other.is_zero() {
    //         eprintln!("Warning: Division by zero");
    //         return Fraction::zero();
    //     }
    //     return Fraction::new(self, other);
    // }

    pub fn pow(self, exp: u32) -> Self {
        let mut base = self;
        let mut exp = exp;
        let mut result = BigInt::one();

        while exp > 0 {
            if exp % 2 == 1 {
            result = result * base.clone();
            }
            base = base.clone() * base;
            exp /= 2;
        }

        result
    }

    pub fn mod_pow(self, exp: u32, modulus: Self) -> Self {
        if modulus.digits.is_empty() || (modulus.digits.len() == 1 && modulus.digits[0] == 0) {
            panic!("Modulus cannot be zero");
        }
        let mut base = self % modulus.clone();
        let mut exp = exp;
        let mut result = BigInt::one() % modulus.clone();
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base.clone()) % modulus.clone();
            }
            base = (base.clone() * base) % modulus.clone();
            exp /= 2;
        }
        
        result
    }

    pub fn is_zero(&self) -> bool {
        self.digits.is_empty() || (self.digits.len() == 1 && self.digits[0] == 0)
    }

    pub fn is_one(&self) -> bool {
        self.digits.len() == 1 && self.digits[0] == 1
    }

    pub fn is_negative(&self) -> bool {
        self.digits.first().map_or(false, |&digit| digit < 0)
    }

    pub fn gcd(&self, other: &Self) -> Self {
        let mut a = self.clone().abs();
        let mut b = other.clone().abs();

        while !b.is_zero() {
            let temp = b.clone();
            b = a % b;
            a = temp;
        }

        a
    }

    pub fn abs(mut self) -> Self {
        self.sign = false;
        self
    }

    pub fn factorial(self) -> Self {
        if self.is_negative() {
            eprintln!("Warning: Factorial of a negative number is undefined");
            return BigInt::zero();
        }
        let mut result = BigInt::one();
        let mut i = BigInt::one();
        while i <= self.clone() {
            result = result * i.clone();
            i = i + BigInt::one();
        }
        result
    }
}


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

pub fn miller_rabin(n: i128) -> bool {
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

pub fn find_factor(n: i128, factors: &mut Vec<i128>) {
    if miller_rabin(n) {
        factors.push(n);
        return;
    }
    let divisor = pollard_rho(n, 1);
    find_factor(divisor, factors);
    find_factor(n / divisor, factors);
}
