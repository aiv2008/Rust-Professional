/*
    Rotate Matrix 90 Degrees
    Given a 2D matrix, rotate it 90 degrees in place. 
    You need to perform the rotation without using any additional matrix storage.

    You need to implement the function `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`.
    The function should rotate the input matrix in place.

    Hint: Consider rotating the matrix layer by layer, starting from the outermost layer and working your way inward.
*/

use std::fmt::{self, Display, Formatter};

pub fn rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>) {
    // TODO: Implement the logic to rotate the matrix 90 degrees in place
    let mut result = Vec::<Vec::<i32>>::new();
    for i in 0..matrix[0].len(){
        let mut temp: Vec<i32> = Vec::<i32>::new();
        let n = matrix.len();
        for j in 0..n{
            println!("{}",matrix[n-j-1][i]);
            temp.push( matrix[n-j-1][i]);
        }
        result.push(temp);
    }
    let mut i = 0;
    let mut j = 0;
    let mut n = 0;
    let mut kk = 0;
    while n < matrix.len() && i < result.len(){
        while kk < matrix[0].len() && j < result[0].len(){
            matrix[n][kk] = result[n][kk];
            kk += 1;
            j+=1;
        }
        if kk == result[0].len(){
            for i in kk..result[0].len(){
                matrix[n].push(result[n][i]);
            }
        }else if j == result[0].len(){
            for i in j..matrix[0].len(){
                matrix[n].pop();
            }
        }
        n += 1;
        i += 1;
    }

    if n == matrix.len(){
        for i in n..result.len(){
            matrix.push(result[i].clone());
        }
    }else if i == result.len(){
        for j in i..matrix.len(){
            matrix.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_1() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
        ]);
    }

    #[test]
    fn test_rotate_matrix_2() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![3, 1],
            vec![4, 2],
        ]);
    }

    #[test]
    fn test_rotate_matrix_3() {
        let mut matrix = vec![
            vec![1],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![1],
        ]);
    }

    #[test]
    fn test_rotate_matrix_4() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![5, 3, 1],
            vec![6, 4, 2],
        ]);
    }
}
