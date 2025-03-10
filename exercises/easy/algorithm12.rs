/*
    Palindrome Check
    Given a string, check if it is a palindrome (i.e., it reads the same forward and backward).
    The solution should ignore case differences and non-alphabetical characters.

    You need to implement the function `is_palindrome(s: String) -> bool`.
    The function should return `true` if the string is a palindrome, and `false` otherwise.
    
    Hint: Consider normalizing the string by converting it to lowercase and removing non-alphabetical characters before checking.
*/

use std::fmt::{self, Display, Formatter};

pub fn is_palindrome(s: String) -> bool {
    // TODO: Implement the logic to check if the string is a palindrome
    // let mut stack: Stack<u8> = Stack::new();
	let mut v: Vec<u8> = Vec::new();
	for c in s.chars(){
		if ((c as u8) <= 90 && (c as u8) >= 65) || ((c as u8) <= 122 && (c as u8) >= 97){
			if (c as u8) <= 90 && (c as u8) >= 65{
				v.push((c as u8) + 32);
				// stack.push((c as u8) + 32);
			}else{
				v.push(c as u8);
				// stack.push(c as u8);
			}
		}
	}
	
    let mut n = 0;
    if v.len()%2==0{
        n = v.len()/2;
    }else{
        n = (v.len()-1)/2;
    }
    let mut j = 0;
    for i in 0..n{
        println!("vi={}, vn-i={}", v[i], v[v.len()-i-1]);
        if v[i] != v[v.len()-i-1]{
            break;
        }else{
            j += 1;
        }
    }
    println!("i={}", j);
    if v.len()%2==0{
        if j != v.len()/2{
            false
        }else{
            true
        }
    }else{
        if j != (v.len()-1)/2{
            false
        }else{
            true
        }
    }
    // true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_1() {
        let s = "A man, a plan, a canal, Panama".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_2() {
        let s = "Racecar".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_3() {
        let s = "Hello, World!".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_palindrome_4() {
        let s = "No 'x' in Nixon".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_5() {
        let s = "Was it a car or a cat I saw?".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }
}
