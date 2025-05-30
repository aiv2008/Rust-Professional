/*
    Sum of Two Integers
    Given two integers, calculate their sum without using the `+` operator. 
    You need to implement the function `get_sum(a: i32, b: i32) -> i32`.
    The function should return the sum of the two integers `a` and `b`.

    Hint: You can solve this problem using bitwise operations.
*/

use std::fmt::{self, Display, Formatter};

pub fn get_sum(a: i32, b: i32) -> i32 {
    let mut ans: i32 = 0;
    let mut b_temp =b;
    let mut a_temp =a;
    while b_temp != 0{
        ans = a_temp ^ b_temp;
        b_temp = (a_temp &b_temp) <<1;
        a_temp = ans;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_1() {
        let result = get_sum(1, 2);
        println!("Sum of 1 and 2: {}", result);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_sum_2() {
        let result = get_sum(-1, 1);
        println!("Sum of -1 and 1: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sum_3() {
        let result = get_sum(100, 200);
        println!("Sum of 100 and 200: {}", result);
        assert_eq!(result, 300);
    }

    #[test]
    fn test_sum_4() {
        let result = get_sum(-50, -50);
        println!("Sum of -50 and -50: {}", result);
        assert_eq!(result, -100);
    }

    #[test]
    fn test_sum_5() {
        let result = get_sum(0, 0);
        println!("Sum of 0 and 0: {}", result);
        assert_eq!(result, 0);
    }
}
