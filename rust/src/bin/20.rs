use std::fmt::Display;

/**
n!  means n x (n - 1) x ... x 3 x 2 x 1.

For example, 10! = 10 x 9 x ... x 3 x 2 x 1 == 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!
*/
use num::bigint::BigInt;

fn main() {
    let answer = sum_of_factorial_digits(100);
    println!("{}", answer);
}

fn sum_of_factorial_digits(n: u8) -> u32 {
    let factorial: BigInt = (1..n).product();
    sum_of_digits(factorial)
}

fn sum_of_digits<T>(n: T) -> u32
where
    T: Display,
{
    n.to_string()
        .chars()
        .map(|c| c.to_string().parse::<u32>().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_factorial_digits() {
        assert_eq!(sum_of_factorial_digits(10), 27);
    }

    #[test]
    fn test_sum_of_digits() {
        assert_eq!(sum_of_digits(3628800), 27);
    }
}
