/*
    Merge Intervals
    Given an array of intervals where each interval is represented by a pair of integers [start, end], 
    merge all overlapping intervals and return a list of non-overlapping intervals.
    
    The intervals are inclusive, meaning the interval [start, end] includes both start and end points.
    
    You need to implement the function `merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.
    The function should return a vector containing all the merged intervals.

    Hint: You can start by sorting the intervals by their starting point and then merge them one by one.
*/

use std::{cmp::{self, max}, fmt::{self, Display, Formatter}};


fn sort(array: Vec<Vec<i32>>)->Vec<Vec<i32>>{
	//TODO
    if array.len() <= 1  {
        return array;
    }
    
    let mut result = Vec::<Vec<i32>>::new();
    for a in array{
        result.push(a);
    }
    // result.push(array[0].clone());
    for i in 1..result.len(){
        for j in 0..i{
            let k = i-j;
            if result[k][0] < result[k-1][0]{
                let temp = result[k].clone();
                result[k] = result[k-1].clone();
                result[k-1] = temp.clone();
                // result.insert( k-1, array[k].clone());
            }
            // else{
            //     result.push(array[k].clone());
            // }
        }
    }
    result
}

fn merge(intervals1: Vec<i32>,intervals2: Vec<i32>) -> Vec<Vec<i32>>{
    let mut result = Vec::<Vec<i32>>::new();
    if intervals1[1] < intervals2[0]{
        result.push(intervals1);
        result.push(intervals2);
    }else{
        result.push(vec![intervals1[0], max(intervals1[1], intervals2[1])]);
    }
    result
}

pub fn merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // TODO: Implement the logic to merge overlapping intervals
    let v = sort(intervals);
    let mut result = Vec::<Vec<i32>>::new();
    if v.is_empty(){
        Vec::<Vec<i32>>::new()
    }else{
        result.push(v[0].clone());
        for i in 1..v.len(){
            let temp = merge(result[result.len()-1].clone(),v[i].clone());
            result.pop();
            for a in temp{
                result.push(a);
            }
        }
        result
    }
    // Vec::new() // Placeholder return value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals_1() {
        let intervals = vec![
            vec![1, 3],
            vec![2, 6],
            vec![8, 10],
            vec![15, 18]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 6],
            vec![8, 10],
            vec![15, 18]
        ]);
    }

    #[test]
    fn test_merge_intervals_2() {
        let intervals = vec![
            vec![1, 4],
            vec![4, 5]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 5]
        ]);
    }

    #[test]
    fn test_merge_intervals_3() {
        let intervals = vec![
            vec![1, 4],
            vec![0, 4]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![0, 4]
        ]);
    }

    #[test]
    fn test_merge_intervals_4() {
        let intervals = vec![
            vec![1, 10],
            vec![2, 6],
            vec![8, 10]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 10]
        ]);
    }

    #[test]
    fn test_merge_intervals_5() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![4, 7],
            vec![8, 10]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 2],
            vec![3, 7],
            vec![8, 10]
        ]);
    }
}