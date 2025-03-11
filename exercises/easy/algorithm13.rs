/*
    Anagram Check
    Given two strings, check if they are anagrams of each other. 
    Anagrams are words or phrases formed by rearranging the letters of another, 
    using all the original letters exactly once. 
    The strings may contain spaces or punctuation, but you need to ignore them while checking.

    You need to implement the function `are_anagrams(s1: String, s2: String) -> bool`.
    The function should return `true` if the two strings are anagrams, and `false` otherwise.

    Hint: Consider normalizing the strings by removing non-alphabetical characters and converting to lowercase before checking.
*/

use std::fmt::{self, Display, Formatter};

pub fn are_anagrams(s1: String, s2: String) -> bool {
    // TODO: Implement the logic to check if two strings are anagrams
    let mut v1: Vec<u8> = Vec::new();

    let mut v2: Vec<u8> = Vec::new();
    for c in s1.chars(){
        if ((c as u8) <= 90 && (c as u8) >= 65) || ((c as u8) <= 122 && (c as u8) >= 97){
			if (c as u8) <= 90 && (c as u8) >= 65{
				v1.push((c as u8) + 32);
				// stack.push((c as u8) + 32);
			}else{
				v1.push(c as u8);
				// stack.push(c as u8);
			}
		}
     }
     for c in s2.chars(){
        if ((c as u8) <= 90 && (c as u8) >= 65) || ((c as u8) <= 122 && (c as u8) >= 97){
			if (c as u8) <= 90 && (c as u8) >= 65{
				v2.push((c as u8) + 32);
				// stack.push((c as u8) + 32);
			}else{
				v2.push(c as u8);
				// stack.push(c as u8);
			}
		}
     }
     sort(&mut v1);
     sort(&mut v2);
     if v1.len() != v2.len(){
        false
     }else{
        let mut j = 0;
        for i in 0..v1.len(){
            if v1[i] != v2[i]{
                break;
            }
            j += 1;
        }

        if j != v1.len() || j != v2.len(){
            false
        }else{
            true
        }
     }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_1() {
        let s1 = "listen".to_string();
        let s2 = "silent".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_2() {
        let s1 = "evil".to_string();
        let s2 = "vile".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_3() {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_anagram_4() {
        let s1 = "Clint Eastwood".to_string();
        let s2 = "Old West Action".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_5() {
        let s1 = "Astronomer".to_string();
        let s2 = "Moon starer".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }
}
