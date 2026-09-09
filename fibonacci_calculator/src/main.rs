use std::io;
use num_bigint::BigUint;

fn multiply_matrix(a: &[[BigUint; 2]; 2], b: &[[BigUint; 2]; 2]) -> [[BigUint; 2]; 2] {
    let top_left = &a[0][0] * &b[0][0] + &a[0][1] * &b[1][0];
    let top_right = &a[0][0] * &b[0][1] + &a[0][1] * &b[1][1];
    let bottom_left = &a[1][0] * &b[0][0] + &a[1][1] * &b[1][0];
    let bottom_right = &a[1][0] * &b[0][1] + &a[1][1] * &b[1][1];

    [[top_left, top_right], [bottom_left, bottom_right]]
}

// 2. The Exponentiation by Squaring algorithm
fn matrix_power(mut matrix: [[BigUint; 2]; 2], mut n: u64) -> [[BigUint; 2]; 2] {
    let mut result = [
        [BigUint::from(1u32), BigUint::from(0u32)],
        [BigUint::from(0u32), BigUint::from(1u32)],
    ];

    while n > 0 {
        if n % 2 == 1 {
            result = multiply_matrix(&result, &matrix);
        }
        matrix = multiply_matrix(&matrix, &matrix);
        n /= 2;
    }

    result
}

fn fibonacci_fast(n: u64) -> BigUint {
    if n == 0 {
        return BigUint::from(0u32);
    }

    let base_matrix = [
        [BigUint::from(1u32), BigUint::from(1u32)],
        [BigUint::from(1u32), BigUint::from(0u32)],
    ];

    let result_matrix = matrix_power(base_matrix, n - 1);
    result_matrix[0][0].clone()
}

fn main() {
    println!("Input Fibonacci index:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");
    let n: u64 = input.trim().parse().expect("Input must be a valid positive number!");

    let answer = fibonacci_fast(n);
    let answer_str = answer.to_string(); // Compute the string once
    
    println!();
    println!("{}. Fibonacci number is below with {} digits:", n, answer_str.len());
    println!();
    println!("{}", answer_str);
}
