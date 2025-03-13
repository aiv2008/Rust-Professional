/*
    Find Intersection of Two Arrays
    Given two arrays, find the intersection of the arrays and return the elements of the intersection (without duplicates).
    The result should not contain any duplicate elements.

    You need to implement the function `intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>`.
    The function should return a vector containing all the elements that are in both arrays.

    Hint: You can solve this problem using sorting, hash sets, or the two-pointer technique.
*/

use std::fmt::{self, Display, Formatter};

fn distinct(merge_v: Vec<i32>)-> Vec<i32>{
    let mut result = Vec::<i32>::new();
    let mut i = 0;
    let mut j=1;
    let mut l = i;
    if merge_v.len()>1{
         while l< merge_v.len(){
            println!("i={}", i);
            j = i + 1;
            while j <  merge_v.len(){
                if merge_v[i] == merge_v[j]{
                    if!result.contains(&merge_v[i]){
                        result.push(merge_v[i]);
                    }
                    i = j + 1;
                    break;
                }else{
                    // result.push(merge_v[i]);
                    j += 1;
                }
            }
            if i <= l + 1{
                i += 1;
                l += 1;
            }else{
                l = i;
            }
        }
    }
    result
}

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    // TODO: Implement the logic to find the intersection of two arrays
    let mut v1 = Vec::<i32>::new();
    let mut v2 = Vec::<i32>::new();
    for a in nums1 {
        v1.push(a);
    }
    for a in nums2 {
        v2.push(a);
    }
    v1.sort();
    v2.sort();
    let mut dis_v1 = Vec::<i32>::new();
    for a in v1{
        if !dis_v1.contains(&a){
            dis_v1.push(a);
        }
    }
    let mut dis_v2 = Vec::<i32>::new();
    for a in v2{
        if !dis_v2.contains(&a){
            dis_v2.push(a);
        }
    }
    
    let mut merge_v = Vec::<i32>::new();
    for a in dis_v1 {
        merge_v.push(a);
    }
    for a in dis_v2 {
        merge_v.push(a);
    } 
    merge_v.sort();
    println!("merge={:?}",merge_v );
    let result = distinct(merge_v);
    result
    // Vec::new() // Placeholder return value
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn test_intersection_2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![4, 9]);
    }

    #[test]
    fn test_intersection_3() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![4, 5, 6];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_intersection_4() {
        let nums1 = vec![1, 1, 1];
        let nums2 = vec![1, 1, 1];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_intersection_5() {
        let nums1 = vec![10, 20, 30];
        let nums2 = vec![30, 40, 50];
        let result = intersection(nums1, nums2);
        println!("Intersection: {:?}", result);
        assert_eq!(result, vec![30]);
    }
}
